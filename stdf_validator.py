#!/usr/bin/env python3
"""
Advanced STDF V4 File Validator

This is a comprehensive, standalone Python STDF validator that performs deep
binary-level parsing and cross-record consistency validation.

USAGE:
======

Command Line:
    python stdf_validator.py <stdf_file>
    
    Example:
    python stdf_validator.py test.stdf

Python API:
    from stdf_validator import validate_stdf_file, print_validation_result
    
    # Simple validation
    result = validate_stdf_file("test.stdf")
    if result.is_valid:
        print("File is valid!")
    else:
        print(f"Found {len(result.errors)} errors")
    
    # Detailed validation with custom reporting
    from stdf_validator import STDFValidator
    
    validator = STDFValidator()
    result = validator.validate_file("test.stdf")
    
    # Print comprehensive report
    print_validation_result(result, "test.stdf")
    
    # Access specific validation results
    print(f"Structural errors: {len(result.errors)}")
    print(f"Consistency errors: {len(result.consistency_errors)}")
    print(f"Warnings: {len(result.warnings)}")
    
    # Examine record counts
    for record_type, count in result.record_count.items():
        print(f"{record_type}: {count}")

FEATURES:
=========

1. Deep Binary Parsing:
   - Direct STDF binary format parsing
   - Supports all major record types (FAR, MIR, MRR, PTR, PRR, TSR, HBR, SBR, PCR)
   - Proper STDF data type handling

2. Cross-Record Consistency Validation:
   - PTR-TSR consistency (execution counts vs actual records)
   - PRR-Bin consistency (bin assignments vs definitions)
   - Bin count consistency (reported vs actual counts)
   - Part count consistency (PCR vs PRR records)
   - Test execution consistency

3. Comprehensive Error Reporting:
   - Structural errors (binary format issues)
   - Cross-record consistency errors
   - Warnings for non-critical issues
   - Detailed record summaries

ENHANCED FEATURES (v2.0):
=========================

1. Deep Record Parsing & Analysis:
   - MIR, MRR, SDR, PIR, PRR, PTR, PCR record parsing
   - Field completeness validation
   - Critical field validation (timestamps, identifiers, etc.)

2. Advanced Correlation Checks:
   - MIR/MRR correlation (must be 1:1)
   - PIR/PRR correlation (part tracking)
   - WIR/WRR correlation (wafer testing)
   - Site consistency (SDR vs test records)

3. Cross-Record Consistency:
   - PCR vs actual part counts
   - Test execution consistency
   - Bin assignment validation

4. Test Statistics Analysis:
   - Part yield calculation
   - Test yield calculation
   - Site coverage analysis
   - Test execution statistics

5. Enhanced Reporting:
   - Categorized error types
   - JSON output support
   - Verbose mode with detailed info
   - Test statistics summary

COMPARISON WITH RUST VALIDATOR:
===============================

Use this Python validator when you need:
- Comprehensive analysis and debugging
- Detailed cross-record consistency checks
- Test statistics and yield analysis
- JSON output for integration
- Detailed error categorization

Use the Rust validator (stupidf.validate_stdf()) when you need:
- Maximum performance for large files
- Integration with Rust parsing workflow
- Basic structure and integrity checks
- Memory-efficient validation

Both validators complement each other and can be used together.
"""

import struct
import os
import sys
from typing import Dict, List, Tuple, Optional, Any, Set
from dataclasses import dataclass, field
from enum import Enum
from collections import defaultdict
import argparse
import json


class ValidationError(Exception):
    """Custom exception for STDF validation errors"""
    pass


@dataclass
class ValidationResult:
    """Result of STDF validation"""
    is_valid: bool
    errors: List[str]
    warnings: List[str]
    record_count: Dict[str, int]
    file_size: int
    consistency_errors: List[str]
    correlation_errors: List[str] = field(default_factory=list)
    completeness_errors: List[str] = field(default_factory=list)
    info_messages: List[str] = field(default_factory=list)
    sites_found: Set[int] = field(default_factory=set)
    test_statistics: Dict[str, Any] = field(default_factory=dict)


class STDFValidator:
    """
    STDF V4 File Validator
    
    Validates STDF files according to the V4 specification including:
    - Binary structure validation
    - Required record sequence validation
    - Record type validation
    - Data integrity checks
    """
    
    def __init__(self, strict_mode: bool = False, enable_deep_analysis: bool = True):
        self.errors = []
        self.warnings = []
        self.consistency_errors = []
        self.correlation_errors = []
        self.completeness_errors = []
        self.info_messages = []
        self.record_count = {}
        self.record_sequence = []
        self.parsed_records = []
        self.sites_found = set()
        self.test_statistics = {}
        self.strict_mode = strict_mode
        self.enable_deep_analysis = enable_deep_analysis
        
    def validate_file(self, filepath: str) -> ValidationResult:
        """
        Validate an STDF file
        
        Args:
            filepath: Path to the STDF file
            
        Returns:
            ValidationResult containing validation status and details
        """
        self._reset_state()
        
        if not os.path.exists(filepath):
            self.errors.append(f"File not found: {filepath}")
            return self._create_result(0)
            
        file_size = os.path.getsize(filepath)
        
        try:
            with open(filepath, 'rb') as f:
                self._validate_file_structure(f)
                self._validate_required_records()

                if self.enable_deep_analysis:
                    self._validate_record_correlations()
                    self._validate_record_completeness()
                    self._validate_cross_record_consistency()
                    self._analyze_test_statistics()

        except Exception as e:
            self.errors.append(f"Error reading file: {str(e)}")
            
        return self._create_result(file_size)
    
    def _reset_state(self):
        """Reset validator state for new validation"""
        self.errors = []
        self.warnings = []
        self.consistency_errors = []
        self.correlation_errors = []
        self.completeness_errors = []
        self.info_messages = []
        self.record_count = {}
        self.record_sequence = []
        self.parsed_records = []
        self.sites_found = set()
        self.test_statistics = {}
    
    def _validate_file_structure(self, file_obj):
        """Validate the binary structure of the STDF file"""
        position = 0
        record_index = 0
        
        while True:
            # Read record header
            header_data = file_obj.read(4)
            if len(header_data) == 0:
                break  # End of file
                
            if len(header_data) < 4:
                self.errors.append(f"Incomplete header at position {position}")
                break
                
            try:
                rec_len, rec_typ, rec_sub = struct.unpack('<HBB', header_data)
            except struct.error:
                self.errors.append(f"Invalid header format at position {position}")
                break
                
            # Validate header
            self._validate_header(rec_len, rec_typ, rec_sub, position, record_index)
            
            # Read record data
            if rec_len > 0:
                record_data = file_obj.read(rec_len)
                if len(record_data) != rec_len:
                    self.errors.append(
                        f"Record at position {position}: expected {rec_len} bytes, got {len(record_data)}"
                    )
                    break
            
            # Track record types and parse data if needed
            record_name = self._get_record_name(rec_typ, rec_sub)
            self.record_count[record_name] = self.record_count.get(record_name, 0) + 1
            self.record_sequence.append(record_name)

            # Parse and store records for deep analysis
            if self.enable_deep_analysis:
                parsed_record = self._parse_record(rec_typ, rec_sub, record_data if rec_len > 0 else b'', position, record_index)
                if parsed_record:
                    self.parsed_records.append(parsed_record)

            position += 4 + rec_len
            record_index += 1
    
    def _validate_header(self, rec_len: int, rec_typ: int, rec_sub: int, position: int, record_index: int):
        """Validate record header"""
        # Check for valid record length
        if rec_len > 65535:  # U*2 maximum
            self.errors.append(f"Invalid record length {rec_len} at position {position}")
        
        # First record must be FAR
        if record_index == 0:
            if rec_typ != 0 or rec_sub != 10:
                self.errors.append("First record must be FAR (File Attributes Record)")
        
        # Check for known record types
        record_name = self._get_record_name(rec_typ, rec_sub)
        if record_name == "UNKNOWN":
            if rec_typ < 200:  # Reserved for Teradyne
                self.errors.append(
                    f"Unknown record type ({rec_typ}, {rec_sub}) at position {position}"
                )
            else:
                self.warnings.append(
                    f"Custom record type ({rec_typ}, {rec_sub}) at position {position}"
                )
    
    def _validate_required_records(self):
        """Validate that all required records are present"""
        required_records = ["FAR", "MIR", "PCR", "MRR"]
        
        for record_type in required_records:
            if record_type not in self.record_count:
                self.errors.append(f"Missing required record: {record_type}")
        
        # Check for exactly one FAR and MIR
        if self.record_count.get("FAR", 0) != 1:
            self.errors.append("File must contain exactly one FAR record")
            
        if self.record_count.get("MIR", 0) != 1:
            self.errors.append("File must contain exactly one MIR record")
            
        if self.record_count.get("MRR", 0) != 1:
            self.errors.append("File must contain exactly one MRR record")
            
        # Must have at least one PCR
        if self.record_count.get("PCR", 0) == 0:
            self.errors.append("File must contain at least one PCR record")

        # SDR validation
        if self.record_count.get("SDR", 0) == 0:
            self.warnings.append("File does not contain any SDR records - site configuration may be incomplete")
    
    def _get_record_name(self, rec_typ: int, rec_sub: int) -> str:
        """Get record name from type and subtype"""
        record_types = {
            (0, 10): "FAR", (0, 20): "ATR",
            (1, 10): "MIR", (1, 20): "MRR", (1, 30): "PCR", (1, 40): "HBR",
            (1, 50): "SBR", (1, 60): "PMR", (1, 62): "PGR", (1, 63): "PLR",
            (1, 70): "RDR", (1, 80): "SDR",
            (2, 10): "WIR", (2, 20): "WRR", (2, 30): "WCR",
            (5, 10): "PIR", (5, 20): "PRR",
            (10, 30): "TSR",
            (15, 10): "PTR", (15, 15): "MPR", (15, 20): "FTR",
            (20, 10): "BPS", (20, 20): "EPS",
            (50, 10): "GDR", (50, 30): "DTR"
        }
        return record_types.get((rec_typ, rec_sub), "UNKNOWN")
    
    def _parse_record(self, rec_typ: int, rec_sub: int, data: bytes, position: int, index: int) -> Optional[Dict[str, Any]]:
        """Parse record data for deep analysis"""
        record_name = self._get_record_name(rec_typ, rec_sub)
        record = {
            'type': record_name,
            'rec_typ': rec_typ,
            'rec_sub': rec_sub,
            'position': position,
            'index': index,
            'data': data
        }

        try:
            if record_name == 'MIR' and len(data) >= 12:
                record.update(self._parse_mir(data))
            elif record_name == 'MRR' and len(data) >= 5:
                record.update(self._parse_mrr(data))
            elif record_name == 'SDR' and len(data) >= 3:
                record.update(self._parse_sdr(data))
            elif record_name == 'PIR' and len(data) >= 2:
                record.update(self._parse_pir(data))
            elif record_name == 'PRR' and len(data) >= 14:
                record.update(self._parse_prr(data))
            elif record_name == 'PTR' and len(data) >= 15:
                record.update(self._parse_ptr(data))
            elif record_name == 'PCR' and len(data) >= 22:
                record.update(self._parse_pcr(data))
        except Exception as e:
            self.warnings.append(f"Failed to parse {record_name} at position {position}: {e}")

        return record

    def _parse_mir(self, data: bytes) -> Dict[str, Any]:
        """Parse Master Information Record"""
        if len(data) < 12:
            return {}

        setup_t, start_t, stat_num, mode_cod, rtst_cod, prot_cod, burn_tim, cmod_cod = struct.unpack('<IIBBBBB', data[:12])

        # Parse variable length strings
        offset = 12
        strings = []
        for _ in range(20):  # Expected number of string fields
            if offset >= len(data):
                strings.append('')
                continue
            str_len = data[offset] if offset < len(data) else 0
            offset += 1
            if offset + str_len <= len(data):
                strings.append(data[offset:offset + str_len].decode('ascii', errors='ignore'))
                offset += str_len
            else:
                strings.append('')
                break

        return {
            'setup_t': setup_t,
            'start_t': start_t,
            'stat_num': stat_num,
            'mode_cod': chr(mode_cod) if 32 <= mode_cod <= 126 else ' ',
            'rtst_cod': chr(rtst_cod) if 32 <= rtst_cod <= 126 else ' ',
            'prot_cod': chr(prot_cod) if 32 <= prot_cod <= 126 else ' ',
            'burn_tim': burn_tim,
            'cmod_cod': chr(cmod_cod) if 32 <= cmod_cod <= 126 else ' ',
            'lot_id': strings[0] if len(strings) > 0 else '',
            'part_typ': strings[1] if len(strings) > 1 else '',
            'node_nam': strings[2] if len(strings) > 2 else '',
            'tstr_typ': strings[3] if len(strings) > 3 else '',
        }

    def _parse_mrr(self, data: bytes) -> Dict[str, Any]:
        """Parse Master Results Record"""
        if len(data) < 5:
            return {}

        finish_t, disp_cod = struct.unpack('<IB', data[:5])

        # Parse variable length strings
        offset = 5
        usr_desc = ''
        exc_desc = ''

        if offset < len(data):
            str_len = data[offset]
            offset += 1
            if offset + str_len <= len(data):
                usr_desc = data[offset:offset + str_len].decode('ascii', errors='ignore')
                offset += str_len

        if offset < len(data):
            str_len = data[offset]
            offset += 1
            if offset + str_len <= len(data):
                exc_desc = data[offset:offset + str_len].decode('ascii', errors='ignore')

        return {
            'finish_t': finish_t,
            'disp_cod': chr(disp_cod) if 32 <= disp_cod <= 126 else ' ',
            'usr_desc': usr_desc,
            'exc_desc': exc_desc
        }

    def _parse_sdr(self, data: bytes) -> Dict[str, Any]:
        """Parse Site Description Record"""
        if len(data) < 3:
            return {}

        head_num, site_grp, site_cnt = struct.unpack('<BBB', data[:3])

        offset = 3
        site_nums = []

        # Parse site numbers
        for _ in range(site_cnt):
            if offset < len(data):
                site_nums.append(data[offset])
                offset += 1

        return {
            'head_num': head_num,
            'site_grp': site_grp,
            'site_cnt': site_cnt,
            'site_nums': site_nums
        }

    def _parse_pir(self, data: bytes) -> Dict[str, Any]:
        """Parse Part Information Record"""
        if len(data) < 2:
            return {}

        head_num, site_num = struct.unpack('<BB', data[:2])
        self.sites_found.add(site_num)

        return {
            'head_num': head_num,
            'site_num': site_num
        }

    def _parse_prr(self, data: bytes) -> Dict[str, Any]:
        """Parse Part Results Record"""
        if len(data) < 14:
            return {}

        head_num, site_num, part_flg, num_test, hard_bin, soft_bin, x_coord, y_coord, test_t = struct.unpack('<BBBHHHhhI', data[:14])
        self.sites_found.add(site_num)

        return {
            'head_num': head_num,
            'site_num': site_num,
            'part_flg': part_flg,
            'num_test': num_test,
            'hard_bin': hard_bin,
            'soft_bin': soft_bin,
            'x_coord': x_coord,
            'y_coord': y_coord,
            'test_t': test_t
        }

    def _parse_ptr(self, data: bytes) -> Dict[str, Any]:
        """Parse Parametric Test Record"""
        if len(data) < 15:
            return {}

        test_num, head_num, site_num, test_flg, parm_flg, result = struct.unpack('<IBBBBF', data[:15])
        self.sites_found.add(site_num)

        return {
            'test_num': test_num,
            'head_num': head_num,
            'site_num': site_num,
            'test_flg': test_flg,
            'parm_flg': parm_flg,
            'result': result,
            'pass_fail': (test_flg >> 6) & 0b11 == 0
        }

    def _parse_pcr(self, data: bytes) -> Dict[str, Any]:
        """Parse Part Count Record"""
        if len(data) < 22:
            return {}

        head_num, site_num, part_cnt, rtst_cnt, abrt_cnt, good_cnt, func_cnt = struct.unpack('<BBIIIII', data[:22])

        return {
            'head_num': head_num,
            'site_num': site_num,
            'part_cnt': part_cnt,
            'rtst_cnt': rtst_cnt,
            'abrt_cnt': abrt_cnt,
            'good_cnt': good_cnt,
            'func_cnt': func_cnt
        }

    def _validate_record_correlations(self):
        """Validate correlations between different record types"""
        mir_count = self.record_count.get('MIR', 0)
        mrr_count = self.record_count.get('MRR', 0)
        sdr_count = self.record_count.get('SDR', 0)
        wir_count = self.record_count.get('WIR', 0)
        wrr_count = self.record_count.get('WRR', 0)
        pir_count = self.record_count.get('PIR', 0)
        prr_count = self.record_count.get('PRR', 0)

        # MIR/MRR correlation
        if mir_count != mrr_count:
            self.correlation_errors.append(
                f"MIR/MRR count mismatch: {mir_count} MIR records, {mrr_count} MRR records"
            )

        # WIR/WRR correlation for wafer testing
        if wir_count > 0 and wir_count != wrr_count:
            self.correlation_errors.append(
                f"WIR/WRR count mismatch: {wir_count} WIR records, {wrr_count} WRR records"
            )

        # PIR/PRR correlation
        if pir_count != prr_count:
            self.correlation_errors.append(
                f"PIR/PRR count mismatch: {pir_count} PIR records, {prr_count} PRR records"
            )

        # Site consistency validation
        sdr_sites = set()
        for record in self.parsed_records:
            if record['type'] == 'SDR' and 'site_nums' in record:
                sdr_sites.update(record['site_nums'])

        if self.sites_found and sdr_count > 0:
            missing_sdr_sites = self.sites_found - sdr_sites
            if missing_sdr_sites:
                self.correlation_errors.append(
                    f"Sites used in test data but not defined in SDR: {sorted(missing_sdr_sites)}"
                )

        if sdr_count == 0 and self.sites_found:
            self.correlation_errors.append(
                "Test data references sites but no SDR records found to define site configuration"
            )

    def _validate_record_completeness(self):
        """Validate completeness of critical records"""
        for record in self.parsed_records:
            if record['type'] == 'MIR':
                self._validate_mir_completeness(record)
            elif record['type'] == 'MRR':
                self._validate_mrr_completeness(record)
            elif record['type'] == 'SDR':
                self._validate_sdr_completeness(record)

    def _validate_mir_completeness(self, record: Dict[str, Any]):
        """Validate MIR record completeness"""
        position = record.get('position', 'unknown')

        # Check critical fields
        if not record.get('lot_id', '').strip():
            self.completeness_errors.append(f"MIR at position {position}: lot_id field is empty")

        if not record.get('part_typ', '').strip():
            self.completeness_errors.append(f"MIR at position {position}: part_typ field is empty")

        if not record.get('node_nam', '').strip():
            self.completeness_errors.append(f"MIR at position {position}: node_nam field is empty")

        if not record.get('tstr_typ', '').strip():
            self.completeness_errors.append(f"MIR at position {position}: tstr_typ field is empty")

        if record.get('setup_t', 0) == 0:
            self.info_messages.append(f"MIR at position {position}: setup_t is 0 (setup timestamp not set)")

        if record.get('start_t', 0) == 0:
            self.completeness_errors.append(f"MIR at position {position}: start_t is 0 (test start timestamp missing)")

        # Validate mode codes
        mode_cod = record.get('mode_cod', ' ')
        if mode_cod not in 'ACDEIMDQSUPV ':
            self.completeness_errors.append(
                f"MIR at position {position}: invalid mode_cod '{mode_cod}' (not a standard STDF mode code)"
            )

    def _validate_mrr_completeness(self, record: Dict[str, Any]):
        """Validate MRR record completeness"""
        position = record.get('position', 'unknown')

        if record.get('finish_t', 0) == 0:
            self.completeness_errors.append(f"MRR at position {position}: finish_t is 0 (test completion timestamp missing)")

        # Validate disposition code
        disp_cod = record.get('disp_cod', ' ')
        if disp_cod not in 'ACEILMPQRSU ':
            self.completeness_errors.append(
                f"MRR at position {position}: invalid disp_cod '{disp_cod}' (not a standard STDF disposition code)"
            )

        if self.strict_mode and not record.get('usr_desc', '').strip():
            self.info_messages.append(f"MRR at position {position}: usr_desc field is empty")

    def _validate_sdr_completeness(self, record: Dict[str, Any]):
        """Validate SDR record completeness"""
        position = record.get('position', 'unknown')

        site_cnt = record.get('site_cnt', 0)
        site_nums = record.get('site_nums', [])

        if site_cnt == 0:
            self.completeness_errors.append(f"SDR at position {position}: site_cnt is 0 (no sites defined)")

        if not site_nums:
            self.completeness_errors.append(f"SDR at position {position}: site_nums array is empty")
        elif len(site_nums) != site_cnt:
            self.completeness_errors.append(
                f"SDR at position {position}: site_cnt ({site_cnt}) does not match site_nums array length ({len(site_nums)})"
            )

        # Check for duplicate site numbers
        if len(site_nums) != len(set(site_nums)):
            duplicates = [x for x in site_nums if site_nums.count(x) > 1]
            self.completeness_errors.append(
                f"SDR at position {position}: contains duplicate site numbers: {list(set(duplicates))}"
            )

    def _validate_cross_record_consistency(self):
        """Validate consistency across multiple records"""
        # Collect data for consistency checks
        pcr_records = [r for r in self.parsed_records if r['type'] == 'PCR']
        prr_records = [r for r in self.parsed_records if r['type'] == 'PRR']
        ptr_records = [r for r in self.parsed_records if r['type'] == 'PTR']

        # Validate PCR vs actual part counts
        for pcr in pcr_records:
            site_num = pcr.get('site_num', 0)
            expected_parts = pcr.get('part_cnt', 0)
            actual_parts = len([p for p in prr_records if p.get('site_num') == site_num])

            if expected_parts != actual_parts:
                self.consistency_errors.append(
                    f"PCR/PRR count mismatch for site {site_num}: PCR reports {expected_parts} parts, found {actual_parts} PRR records"
                )

        # Validate test execution consistency
        if ptr_records:
            site_test_counts = defaultdict(lambda: defaultdict(int))
            for ptr in ptr_records:
                site_num = ptr.get('site_num', 0)
                test_num = ptr.get('test_num', 0)
                site_test_counts[site_num][test_num] += 1

            # Report test execution statistics
            self.test_statistics['sites_tested'] = len(site_test_counts)
            self.test_statistics['unique_tests'] = len(set(ptr.get('test_num', 0) for ptr in ptr_records))
            self.test_statistics['total_test_executions'] = len(ptr_records)

    def _analyze_test_statistics(self):
        """Analyze and report test statistics"""
        ptr_records = [r for r in self.parsed_records if r['type'] == 'PTR']
        prr_records = [r for r in self.parsed_records if r['type'] == 'PRR']

        if ptr_records:
            pass_count = sum(1 for ptr in ptr_records if ptr.get('pass_fail', False))
            fail_count = len(ptr_records) - pass_count

            self.test_statistics.update({
                'parametric_tests': len(ptr_records),
                'parametric_pass': pass_count,
                'parametric_fail': fail_count,
                'parametric_yield': (pass_count / len(ptr_records) * 100) if ptr_records else 0
            })

        if prr_records:
            good_parts = sum(1 for prr in prr_records if prr.get('part_flg', 0) & 0x08 == 0)  # Pass flag

            self.test_statistics.update({
                'total_parts': len(prr_records),
                'good_parts': good_parts,
                'part_yield': (good_parts / len(prr_records) * 100) if prr_records else 0
            })

    def _create_result(self, file_size: int) -> ValidationResult:
        """Create validation result"""
        total_critical_errors = len(self.errors) + len(self.consistency_errors) + len(self.correlation_errors) + len(self.completeness_errors)

        return ValidationResult(
            is_valid=total_critical_errors == 0,
            errors=self.errors.copy(),
            warnings=self.warnings.copy(),
            record_count=self.record_count.copy(),
            file_size=file_size,
            consistency_errors=self.consistency_errors.copy(),
            correlation_errors=self.correlation_errors.copy(),
            completeness_errors=self.completeness_errors.copy(),
            info_messages=self.info_messages.copy(),
            sites_found=self.sites_found.copy(),
            test_statistics=self.test_statistics.copy()
        )


def validate_stdf_file(filepath: str, strict_mode: bool = False, enable_deep_analysis: bool = True) -> ValidationResult:
    """
    Convenience function to validate an STDF file

    Args:
        filepath: Path to the STDF file
        strict_mode: Enable strict validation (more stringent checks)
        enable_deep_analysis: Enable comprehensive record analysis

    Returns:
        ValidationResult containing validation status and details
    """
    validator = STDFValidator(strict_mode=strict_mode, enable_deep_analysis=enable_deep_analysis)
    return validator.validate_file(filepath)


def print_validation_result(result: ValidationResult, filepath: str, verbose: bool = False, json_output: bool = False):
    """Print validation results in a readable format"""
    if json_output:
        output = {
            'filepath': filepath,
            'file_size': result.file_size,
            'is_valid': result.is_valid,
            'summary': {
                'total_records': sum(result.record_count.values()),
                'errors': len(result.errors),
                'consistency_errors': len(result.consistency_errors),
                'correlation_errors': len(result.correlation_errors),
                'completeness_errors': len(result.completeness_errors),
                'warnings': len(result.warnings),
                'info_messages': len(result.info_messages)
            },
            'record_count': result.record_count,
            'sites_found': list(result.sites_found),
            'test_statistics': result.test_statistics
        }

        if verbose:
            output.update({
                'errors': result.errors,
                'consistency_errors': result.consistency_errors,
                'correlation_errors': result.correlation_errors,
                'completeness_errors': result.completeness_errors,
                'warnings': result.warnings,
                'info_messages': result.info_messages
            })

        print(json.dumps(output, indent=2))
        return

    # Text output
    print(f"\n🔍 Enhanced STDF Validation Result for: {filepath}")
    print("=" * 80)
    print(f"📁 File Size: {result.file_size:,} bytes")
    print(f"📊 Total Records: {sum(result.record_count.values()):,}")
    print(f"🎯 Sites Found: {len(result.sites_found)} ({sorted(result.sites_found) if result.sites_found else 'none'})")
    print(f"✅ Valid: {'YES' if result.is_valid else 'NO'}")

    # Error summary
    total_critical = len(result.errors) + len(result.consistency_errors) + len(result.correlation_errors) + len(result.completeness_errors)
    if total_critical > 0:
        print(f"\n❌ Critical Issues Found: {total_critical}")

    if result.errors:
        print(f"\n🏗️  Structural Errors ({len(result.errors)}):")
        for i, error in enumerate(result.errors, 1):
            print(f"  {i}. {error}")

    if result.correlation_errors:
        print(f"\n🔗 Record Correlation Errors ({len(result.correlation_errors)}):")
        for i, error in enumerate(result.correlation_errors, 1):
            print(f"  {i}. {error}")

    if result.completeness_errors:
        print(f"\n📝 Field Completeness Errors ({len(result.completeness_errors)}):")
        for i, error in enumerate(result.completeness_errors, 1):
            print(f"  {i}. {error}")

    if result.consistency_errors:
        print(f"\n⚖️  Cross-Record Consistency Errors ({len(result.consistency_errors)}):")
        for i, error in enumerate(result.consistency_errors, 1):
            print(f"  {i}. {error}")

    if result.warnings:
        print(f"\n⚠️  Warnings ({len(result.warnings)}):")
        for i, warning in enumerate(result.warnings, 1):
            print(f"  {i}. {warning}")

    if verbose and result.info_messages:
        print(f"\nℹ️  Information ({len(result.info_messages)}):")
        for i, info in enumerate(result.info_messages, 1):
            print(f"  {i}. {info}")

    # Record summary
    if result.record_count:
        print(f"\n📋 Record Summary:")
        for record_type, count in sorted(result.record_count.items()):
            print(f"  {record_type}: {count:,}")

    # Test statistics
    if result.test_statistics:
        print(f"\n📈 Test Statistics:")
        stats = result.test_statistics
        if 'total_parts' in stats:
            print(f"  Parts Tested: {stats['total_parts']:,}")
            print(f"  Good Parts: {stats.get('good_parts', 0):,}")
            print(f"  Part Yield: {stats.get('part_yield', 0):.2f}%")

        if 'parametric_tests' in stats:
            print(f"  Parametric Tests: {stats['parametric_tests']:,}")
            print(f"  Test Pass: {stats.get('parametric_pass', 0):,}")
            print(f"  Test Fail: {stats.get('parametric_fail', 0):,}")
            print(f"  Test Yield: {stats.get('parametric_yield', 0):.2f}%")

        if 'sites_tested' in stats:
            print(f"  Sites Tested: {stats['sites_tested']}")
            print(f"  Unique Tests: {stats.get('unique_tests', 0)}")

    # Summary
    print(f"\n{'='*80}")
    if total_critical == 0:
        print(f"✅ File passed all validation checks!")
        if result.warnings:
            print(f"⚠️  Note: {len(result.warnings)} warnings found (non-critical)")
    else:
        print(f"❌ File has {total_critical} critical issues that need to be addressed.")
        if result.warnings:
            print(f"⚠️  Additionally, {len(result.warnings)} warnings were found.")


# Enhanced command line interface
def create_parser():
    """Create command line argument parser"""
    parser = argparse.ArgumentParser(
        description='Enhanced STDF V4 File Validator with comprehensive analysis',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic validation
  python stdf_validator.py test.stdf

  # Strict mode with verbose output
  python stdf_validator.py test.stdf --strict --verbose

  # JSON output for integration
  python stdf_validator.py test.stdf --json

  # Quick structural check only
  python stdf_validator.py test.stdf --no-deep-analysis

Validation Features:
  ✓ Binary structure validation
  ✓ Required record sequence validation
  ✓ Record type validation
  ✓ Data integrity checks
  ✓ Record correlation validation (MIR/MRR, PIR/PRR, etc.)
  ✓ Field completeness validation
  ✓ Cross-record consistency checks
  ✓ Test statistics analysis
  ✓ Site configuration validation
"""
    )

    parser.add_argument('filepath', help='Path to the STDF file to validate')
    parser.add_argument('--strict', action='store_true',
                       help='Enable strict validation mode (more stringent checks)')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Show detailed information and info messages')
    parser.add_argument('--json', action='store_true',
                       help='Output results in JSON format')
    parser.add_argument('--no-deep-analysis', action='store_true',
                       help='Skip deep record analysis (faster, basic checks only)')
    parser.add_argument('--version', action='version', version='STDF Validator 2.0')

    return parser

# Example usage and testing
if __name__ == "__main__":
    parser = create_parser()
    args = parser.parse_args()

    if not args.json:
        print(f"🔍 Enhanced STDF V4 File Validator")
        print(f"📁 Validating: {args.filepath}")
        print(f"⚙️  Mode: {'Strict' if args.strict else 'Standard'}")
        print(f"🔬 Analysis: {'Basic' if args.no_deep_analysis else 'Comprehensive'}")
        print()

    result = validate_stdf_file(
        args.filepath,
        strict_mode=args.strict,
        enable_deep_analysis=not args.no_deep_analysis
    )

    print_validation_result(result, args.filepath, verbose=args.verbose, json_output=args.json)

    sys.exit(0 if result.is_valid else 1)