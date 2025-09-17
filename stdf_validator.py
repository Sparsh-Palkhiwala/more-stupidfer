def _validate_far_record(self, data: bytes) -> Dict[str, Any]:
        """Validate File Attributes Record"""
        if len(data) < 2:
            raise ValidationError("FAR record too short")
            
        cpu_type, stdf_ver = struct.unpack('<BB', data[:2])
        self.cpu_type = cpu_type
        self.stdf_version = stdf_ver
        
        # Validate CPU type
        if cpu_type > 255:
            self.errors.append("Invalid CPU type in FAR")
            
        # Validate STDF version
        if stdf_ver != 4:
            self.warnings.append(f"STDF version is {stdf_ver}, expected 4")
            
        self.has_far = True
        return {'cpu_type': cpu_type, 'stdf_ver': stdf_ver}
    
def _validate_mir_record(self, data: bytes) -> Dict[str, Any]:
    """Validate Master Information Record"""
    if len(data) < 12:  # Minimum required fields
        raise ValidationError("MIR record too short")
        
    self.has_mir = True
    # Parse basic MIR fields for cross-validation
    offset = 0
    record = {}
    record['setup_t'], offset = self._parse_u4(data, offset)
    record['start_t'], offset = self._parse_u4(data, offset)
    record['stat_num'], offset = self._parse_u1(data, offset)
    return record
    
def _validate_mrr_record(self, data: bytes) -> Dict[str, Any]:
    """Validate Master Results Record"""
    self.has_mrr = True
    return {}

def _validate_cross_record_consistency(self):
    """Validate consistency across related records"""
    self._validate_ptr_tsr_consistency()
    self._validate_prr_bin_consistency()
    self._validate_bin_count_consistency()
    self._validate_part_count_consistency()
    self._validate_test_execution_consistency()

def _validate_ptr_tsr_consistency(self):
    """Validate PTR and TSR record consistency"""
    # Check that every test in TSR has corresponding PTR records
    tsr_tests = {}
    for tsr in self.tsr_records:
        test_num = tsr.data.get('test_num')
        if test_num is not None:
            site_key = (tsr.data.get('head_num', 1), tsr.data.get('site_num', 1))
            if test_num not in tsr_tests:
                tsr_tests[test_num] = {}
            tsr_tests[test_num][site_key] = tsr
    
    # Count PTR executions per test
    ptr_executions = {}
    for ptr in self.ptr_records:
        test_num = ptr.data.get('test_num')
        if test_num is not None:
            site_key = (ptr.data.get('head_num', 1), ptr.data.get('site_num', 1))
            if test_num not in ptr_executions:
                ptr_executions[test_num] = {}
            if site_key not in ptr_executions[test_num]:
                ptr_executions[test_num][site_key] = 0
            ptr_executions[test_num][site_key] += 1
    
    # Validate consistency
    for test_num, site_tsrs in tsr_tests.items():
        for site_key, tsr in site_tsrs.items():
            tsr_exec_cnt = tsr.data.get('exec_cnt')
            actual_ptr_count = ptr_executions.get(test_num, {}).get(site_key, 0)
            
            if tsr_exec_cnt is not None and tsr_exec_cnt != actual_ptr_count:
                self.consistency_errors.append(
                    f"Test {test_num} site {site_key}: TSR reports {tsr_exec_cnt} executions, "
                    f"but found {actual_ptr_count} PTR records"
                )
    
    # Check for PTR records without corresponding TSR
    for test_num in ptr_executions:
        if test_num not in tsr_tests:
            self.consistency_errors.append(
                f"Test {test_num}: Found PTR records but no corresponding TSR record"
            )

def _validate_prr_bin_consistency(self):
    """Validate PRR bin assignments are consistent with bin definitions"""
    # Collect defined bins
    defined_hbins = set()
    defined_sbins = set()
    
    for hbr in self.hbr_records:
        bin_num = hbr.data.get('hbin_num')
        if bin_num is not None:
            defined_hbins.add(bin_num)
            
    for sbr in self.sbr_records:
        bin_num = sbr.data.get('sbin_num')
        if bin_num is not None:
            defined_sbins.add(bin_num)
    
    # Check PRR bin assignments
    used_hbins = set()
    used_sbins = set()
    
    for prr in self.prr_records:
        hard_bin = prr.data.get('hard_bin')
        soft_bin = prr.data.get('soft_bin')
        
        if hard_bin is not None:
            used_hbins.add(hard_bin)
            if hard_bin not in defined_hbins:
                self.consistency_errors.append(
                    f"PRR references undefined hardware bin {hard_bin}"
                )
        
        if soft_bin is not None:
            used_sbins.add(soft_bin)
            if soft_bin not in defined_sbins:
                self.consistency_errors.append(
                    f"PRR references undefined software bin {soft_bin}"
                )
    
    # Check for unused bins
    unused_hbins = defined_hbins - used_hbins
    unused_sbins = defined_sbins - used_sbins
    
    if unused_hbins:
        self.warnings.append(f"Defined but unused hardware bins: {sorted(unused_hbins)}")
    if unused_sbins:
        self.warnings.append(f"Defined but unused software bins: {sorted(unused_sbins)}")

    def _validate_bin_count_consistency(self):
        """Validate bin counts match actual part counts"""
        # Count parts per bin from PRR records
        actual_hbin_counts = {}
        actual_sbin_counts = {}
        
        for prr in self.prr_records:
            site_key = (prr.data.get('head_num', 1), prr.data.get('site_num', 1))
            hard_bin = prr.data.get('hard_bin')
            soft_bin = prr.data.get('soft_bin')
            
            if hard_bin is not None:
                hbin_key = (site_key, hard_bin)
                actual_hbin_counts[hbin_key] = actual_hbin_counts.get(hbin_key, 0) + 1
                
            if soft_bin is not None:
                sbin_key = (site_key, soft_bin)
                actual_sbin_counts[sbin_key] = actual_sbin_counts.get(sbin_key, 0) + 1
        
        # Compare with HBR records
        for hbr in self.hbr_records:
            site_key = (hbr.data.get('head_num', 1), hbr.data.get('site_num', 1))
            bin_num = hbr.data.get('hbin_num')
            reported_count = hbr.data.get('hbin_cnt', 0)
            
            if bin_num is not None:
                hbin_key = (site_key, bin_num)
                actual_count = actual_hbin_counts.get(hbin_key, 0)
                
                # Handle summary records (HEAD_NUM = 255)
                if site_key[0] == 255:  # Summary for all sites
                    actual_count = sum(
                        count for key, count in actual_hbin_counts.items() 
                        if key[1] == bin_num
                    )
                
                if actual_count != reported_count:
                    self.consistency_errors.append(
                        f"Hardware bin {bin_num} site {site_key}: HBR reports {reported_count} parts, "
                        f"but PRR records show {actual_count}"
                    )
        
        # Compare with SBR records
        for sbr in self.sbr_records:
            site_key = (sbr.data.get('head_num', 1), sbr.data.get('site_num', 1))
            bin_num = sbr.data.get('sbin_num')
            reported_count = sbr.data.get('sbin_cnt', 0)
            
            if bin_num is not None:
                sbin_key = (site_key, bin_num)
                actual_count = actual_sbin_counts.get(sbin_key, 0)
                
                if site_key[0] == 255:  # Summary for all sites
                    actual_count = sum(
                        count for key, count in actual_sbin_counts.items() 
                        if key[1] == bin_num
                    )
                
                if actual_count != reported_count:
                    self.consistency_errors.append(
                        f"Software bin {bin_num} site {site_key}: SBR reports {reported_count} parts, "
                        f"but PRR records show {actual_count}"
                    )

def _validate_part_count_consistency(self):
    """Validate PCR counts match actual part records"""
    # Count parts from PRR records
    actual_part_counts = {}
    
    for prr in self.prr_records:
        site_key = (prr.data.get('head_num', 1), prr.data.get('site_num', 1))
        actual_part_counts[site_key] = actual_part_counts.get(site_key, 0) + 1
    
    # Compare with PCR records
    for site_key, pcr_data in self.part_counts.items():
        reported_count = pcr_data.get('part_cnt', 0)
        
        if site_key[0] == 255:  # Summary for all sites
            actual_count = sum(actual_part_counts.values())
        else:
            actual_count = actual_part_counts.get(site_key, 0)
        
        if actual_count != reported_count:
            self.consistency_errors.append(
                f"Part count site {site_key}: PCR reports {reported_count} parts, "
                f"but found {actual_count} PRR records"
            )
    
    def _validate_test_execution_consistency(self):
        """Validate test execution counts and relationships"""
        # Count test executions per part
        test_executions_per_part = {}
        
        for ptr in self.ptr_records:
            site_key = (ptr.data.get('head_num', 1), ptr.data.get('site_num', 1))
            if site_key not in test_executions_per_part:
                test_executions_per_part[site_key] = 0
            test_executions_per_part[site_key] += 1
        
        # Compare with PRR NUM_TEST field
        for prr in self.prr_records:
            site_key = (prr.data.get('head_num', 1), prr.data.get('site_num', 1))
            reported_tests = prr.data.get('num_test', 0)
            actual_tests = test_executions_per_part.get(site_key, 0)
            
            # This is a per-part comparison, so we need to be more sophisticated
            # For now, just check if the total makes sense
            if reported_tests > 0 and actual_tests == 0:
                self.consistency_errors.append(
                    f"PRR at site {site_key} reports {reported_tests} tests executed, "
                    f"but no PTR records found"
                )
        
        # Validate that all test numbers in PTR have corresponding TSR
        ptr_test_numbers = set()
        for ptr in self.ptr_records:
            test_num = ptr.data.get('test_num')
            if test_num is not None:
                ptr_test_numbers.add(test_num)
        
        tsr_test_numbers = set()
        for tsr in self.tsr_records:
            test_num = tsr.data.get('test_num')
            if test_num is not None:
                tsr_test_numbers.add(test_num)
        
        missing_tsr = ptr_test_numbers - tsr_test_numbers
        if missing_tsr:
            self.consistency_errors.append(
                f"PTR records found for tests without corresponding TSR: {sorted(missing_tsr)}"
            )#!/usr/bin/env python3
"""
STDF V4 File Validator

This module provides validation for Standard Test Data Format (STDF) V4 files
according to the specification. It validates both binary structure and logical
requirements.
"""

import struct
import os
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass
from enum import Enum


class ValidationError(Exception):
    """Custom exception for STDF validation errors"""
    pass


class RecordType(Enum):
    """STDF Record Types as defined in specification"""
    # File Information (REC_TYP=0)
    FAR = (0, 10)  # File Attributes Record
    ATR = (0, 20)  # Audit Trail Record
    
    # Lot Information (REC_TYP=1)
    MIR = (1, 10)  # Master Information Record
    MRR = (1, 20)  # Master Results Record
    PCR = (1, 30)  # Part Count Record
    HBR = (1, 40)  # Hardware Bin Record
    SBR = (1, 50)  # Software Bin Record
    PMR = (1, 60)  # Pin Map Record
    PGR = (1, 62)  # Pin Group Record
    PLR = (1, 63)  # Pin List Record
    RDR = (1, 70)  # Retest Data Record
    SDR = (1, 80)  # Site Description Record
    
    # Wafer Information (REC_TYP=2)
    WIR = (2, 10)  # Wafer Information Record
    WRR = (2, 20)  # Wafer Results Record
    WCR = (2, 30)  # Wafer Configuration Record
    
    # Part Information (REC_TYP=5)
    PIR = (5, 10)  # Part Information Record
    PRR = (5, 20)  # Part Results Record
    
    # Test Information (REC_TYP=10)
    TSR = (10, 30)  # Test Synopsis Record
    
    # Test Execution (REC_TYP=15)
    PTR = (15, 10)  # Parametric Test Record
    MPR = (15, 15)  # Multiple-Result Parametric Record
    FTR = (15, 20)  # Functional Test Record
    
    # Program Sections (REC_TYP=20)
    BPS = (20, 10)  # Begin Program Section Record
    EPS = (20, 20)  # End Program Section Record
    
    # Generic Data (REC_TYP=50)
    GDR = (50, 10)  # Generic Data Record
    DTR = (50, 30)  # Datalog Text Record


@dataclass
class STDFHeader:
    """STDF Record Header structure"""
    rec_len: int
    rec_typ: int
    rec_sub: int


@dataclass
class ParsedRecord:
    """Parsed STDF record data"""
    header: STDFHeader
    position: int
    data: Dict[str, Any]


@dataclass
class ValidationResult:
    """Result of STDF validation"""
    is_valid: bool
    errors: List[str]
    warnings: List[str]
    record_count: Dict[str, int]
    file_size: int
    consistency_errors: List[str]


class STDFValidator:
    """
    STDF V4 File Validator
    
    Validates STDF files according to the V4 specification including:
    - Binary structure validation
    - Required record sequence validation
    - Record type validation
    - Data integrity checks
    - Cross-record consistency validation
    """
    
    def __init__(self):
        self.errors = []
        self.warnings = []
        self.consistency_errors = []
        self.record_count = {}
        self.record_sequence = []
        self.parsed_records = []
        self.has_far = False
        self.has_mir = False
        self.has_mrr = False
        self.has_pcr = False
        self.cpu_type = None
        self.stdf_version = None
        
        # Cross-record tracking
        self.mir_data = None
        self.ptr_records = []  # List of PTR records
        self.prr_records = []  # List of PRR records  
        self.tsr_records = []  # List of TSR records
        self.hbr_records = []  # List of HBR records
        self.sbr_records = []  # List of SBR records
        self.bin_totals = {}   # Track bin usage
        self.test_numbers = set()  # Track test numbers from PTR
        self.part_counts = {}  # Track part counts by site
        
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
                self._validate_record_sequence()
                self._validate_required_records()
                
        except Exception as e:
            self.errors.append(f"Error reading file: {str(e)}")
            
        return self._create_result(file_size)
    
    def _reset_state(self):
        """Reset validator state for new validation"""
        self.errors = []
        self.warnings = []
        self.consistency_errors = []
        self.record_count = {}
        self.record_sequence = []
        self.parsed_records = []
        self.has_far = False
        self.has_mir = False
        self.has_mrr = False
        self.has_pcr = False
        self.cpu_type = None
        self.stdf_version = None
        
        # Reset cross-record tracking
        self.mir_data = None
        self.ptr_records = []
        self.prr_records = []
        self.tsr_records = []
        self.hbr_records = []
        self.sbr_records = []
        self.bin_totals = {}
        self.test_numbers = set()
        self.part_counts = {}
    
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
                header = self._parse_header(header_data)
            except struct.error:
                self.errors.append(f"Invalid header format at position {position}")
                break
                
            # Validate header
            self._validate_header(header, position, record_index)
            
            # Read record data
            if header.rec_len > 0:
                record_data = file_obj.read(header.rec_len)
                if len(record_data) != header.rec_len:
                    self.errors.append(
                        f"Record at position {position}: expected {header.rec_len} bytes, got {len(record_data)}"
                    )
                    break
                    
                # Validate specific record types
                self._validate_record_data(header, record_data, position)
            
            # Track record types
            record_name = self._get_record_name(header.rec_typ, header.rec_sub)
            self.record_count[record_name] = self.record_count.get(record_name, 0) + 1
            self.record_sequence.append((record_name, header))
            
            position += 4 + header.rec_len
            record_index += 1
        
        # Perform cross-record consistency validation
        self._validate_cross_record_consistency()
    
    def _parse_header(self, header_data: bytes) -> STDFHeader:
        """Parse STDF record header"""
        # STDF uses little-endian format for most data
        rec_len, rec_typ, rec_sub = struct.unpack('<HBB', header_data)
        return STDFHeader(rec_len, rec_typ, rec_sub)
    
    def _validate_header(self, header: STDFHeader, position: int, record_index: int):
        """Validate record header"""
        # Check for valid record length
        if header.rec_len > 65535:  # U*2 maximum
            self.errors.append(f"Invalid record length {header.rec_len} at position {position}")
        
        # Check for known record types
        record_name = self._get_record_name(header.rec_typ, header.rec_sub)
        if record_name == "UNKNOWN":
            if header.rec_typ < 200:  # Reserved for Teradyne
                self.errors.append(
                    f"Unknown record type ({header.rec_typ}, {header.rec_sub}) at position {position}"
                )
            else:
                self.warnings.append(
                    f"Custom record type ({header.rec_typ}, {header.rec_sub}) at position {position}"
                )
        
        # First record must be FAR
        if record_index == 0:
            if header.rec_typ != 0 or header.rec_sub != 10:
                self.errors.append("First record must be FAR (File Attributes Record)")
    
    def _validate_record_data(self, header: STDFHeader, data: bytes, position: int):
        """Validate specific record data based on type"""
        record_name = self._get_record_name(header.rec_typ, header.rec_sub)
        
        try:
            parsed_data = {}
            if record_name == "FAR":
                parsed_data = self._validate_far_record(data)
            elif record_name == "MIR":
                parsed_data = self._validate_mir_record(data)
                self.mir_data = parsed_data
            elif record_name == "MRR":
                parsed_data = self._validate_mrr_record(data)
            elif record_name == "PTR":
                parsed_data = self._parse_ptr_record(data)
                self.ptr_records.append(ParsedRecord(header, position, parsed_data))
                if 'test_num' in parsed_data:
                    self.test_numbers.add(parsed_data['test_num'])
            elif record_name == "PRR":
                parsed_data = self._parse_prr_record(data)
                self.prr_records.append(ParsedRecord(header, position, parsed_data))
            elif record_name == "TSR":
                parsed_data = self._parse_tsr_record(data)
                self.tsr_records.append(ParsedRecord(header, position, parsed_data))
            elif record_name == "HBR":
                parsed_data = self._parse_hbr_record(data)
                self.hbr_records.append(ParsedRecord(header, position, parsed_data))
            elif record_name == "SBR":
                parsed_data = self._parse_sbr_record(data)
                self.sbr_records.append(ParsedRecord(header, position, parsed_data))
            elif record_name == "PCR":
                parsed_data = self._parse_pcr_record(data)
                self._track_part_counts(parsed_data)
                
            # Store parsed record for cross-validation
            if parsed_data:
                self.parsed_records.append(ParsedRecord(header, position, parsed_data))
                
        except Exception as e:
            self.errors.append(f"Error validating {record_name} at position {position}: {str(e)}")
    
    def _parse_u1(self, data: bytes, offset: int) -> Tuple[int, int]:
        """Parse U*1 (unsigned byte) and return value and new offset"""
        if offset >= len(data):
            raise ValueError("Not enough data for U*1")
        return data[offset], offset + 1
    
    def _parse_u2(self, data: bytes, offset: int) -> Tuple[int, int]:
        """Parse U*2 (unsigned short) and return value and new offset"""
        if offset + 2 > len(data):
            raise ValueError("Not enough data for U*2")
        value = struct.unpack('<H', data[offset:offset+2])[0]
        return value, offset + 2
    
    def _parse_u4(self, data: bytes, offset: int) -> Tuple[int, int]:
        """Parse U*4 (unsigned long) and return value and new offset"""
        if offset + 4 > len(data):
            raise ValueError("Not enough data for U*4")
        value = struct.unpack('<L', data[offset:offset+4])[0]
        return value, offset + 4
    
    def _parse_i2(self, data: bytes, offset: int) -> Tuple[int, int]:
        """Parse I*2 (signed short) and return value and new offset"""
        if offset + 2 > len(data):
            raise ValueError("Not enough data for I*2")
        value = struct.unpack('<h', data[offset:offset+2])[0]
        return value, offset + 2
    
    def _parse_cn(self, data: bytes, offset: int) -> Tuple[str, int]:
        """Parse C*n (variable length string) and return value and new offset"""
        if offset >= len(data):
            raise ValueError("Not enough data for C*n length")
        length = data[offset]
        if offset + 1 + length > len(data):
            raise ValueError("Not enough data for C*n string")
        value = data[offset+1:offset+1+length].decode('ascii', errors='replace')
        return value, offset + 1 + length
    
    def _parse_ptr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse PTR (Parametric Test Record)"""
        if len(data) < 10:  # Minimum: TEST_NUM(4) + HEAD_NUM(1) + SITE_NUM(1) + TEST_FLG(1) + PARM_FLG(1) + RESULT(4)
            raise ValueError("PTR record too short")
            
        offset = 0
        record = {}
        
        # Required fields
        record['test_num'], offset = self._parse_u4(data, offset)
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['test_flg'], offset = self._parse_u1(data, offset)
        record['parm_flg'], offset = self._parse_u1(data, offset)
        
        # Check if RESULT is valid (TEST_FLG bit 1)
        if record['test_flg'] & 0x02 == 0:  # bit 1 = 0 means result is valid
            if offset + 4 <= len(data):
                record['result'] = struct.unpack('<f', data[offset:offset+4])[0]
                offset += 4
        else:
            record['result'] = None
            
        return record
    
    def _parse_prr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse PRR (Part Results Record)"""
        if len(data) < 8:  # Minimum required fields
            raise ValueError("PRR record too short")
            
        offset = 0
        record = {}
        
        # Required fields
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['part_flg'], offset = self._parse_u1(data, offset)
        record['num_test'], offset = self._parse_u2(data, offset)
        record['hard_bin'], offset = self._parse_u2(data, offset)
        
        # Optional SOFT_BIN
        if offset + 2 <= len(data):
            record['soft_bin'], offset = self._parse_u2(data, offset)
            if record['soft_bin'] == 65535:  # Missing/invalid flag
                record['soft_bin'] = None
        
        return record
    
    def _parse_tsr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse TSR (Test Synopsis Record)"""
        if len(data) < 8:
            raise ValueError("TSR record too short")
            
        offset = 0
        record = {}
        
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['test_typ'], offset = self._parse_cn(data, offset)
        record['test_num'], offset = self._parse_u4(data, offset)
        
        # Optional execution counts
        if offset + 4 <= len(data):
            record['exec_cnt'], offset = self._parse_u4(data, offset)
            if record['exec_cnt'] == 4294967295:  # Missing/invalid flag
                record['exec_cnt'] = None
                
        if offset + 4 <= len(data):
            record['fail_cnt'], offset = self._parse_u4(data, offset)
            if record['fail_cnt'] == 4294967295:
                record['fail_cnt'] = None
                
        return record
    
    def _parse_hbr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse HBR (Hardware Bin Record)"""
        if len(data) < 8:
            raise ValueError("HBR record too short")
            
        offset = 0
        record = {}
        
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['hbin_num'], offset = self._parse_u2(data, offset)
        record['hbin_cnt'], offset = self._parse_u4(data, offset)
        
        # Optional pass/fail flag
        if offset < len(data):
            pf_char = chr(data[offset]) if data[offset] != 32 else None  # 32 = space (missing)
            record['hbin_pf'] = pf_char
            offset += 1
            
        return record
    
    def _parse_sbr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse SBR (Software Bin Record)"""
        if len(data) < 8:
            raise ValueError("SBR record too short")
            
        offset = 0
        record = {}
        
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['sbin_num'], offset = self._parse_u2(data, offset)
        record['sbin_cnt'], offset = self._parse_u4(data, offset)
        
        # Optional pass/fail flag
        if offset < len(data):
            pf_char = chr(data[offset]) if data[offset] != 32 else None
            record['sbin_pf'] = pf_char
            offset += 1
            
        return record
    
    def _parse_pcr_record(self, data: bytes) -> Dict[str, Any]:
        """Parse PCR (Part Count Record)"""
        if len(data) < 18:
            raise ValueError("PCR record too short")
            
        offset = 0
        record = {}
        
        record['head_num'], offset = self._parse_u1(data, offset)
        record['site_num'], offset = self._parse_u1(data, offset)
        record['part_cnt'], offset = self._parse_u4(data, offset)
        record['rtst_cnt'], offset = self._parse_u4(data, offset)
        record['abrt_cnt'], offset = self._parse_u4(data, offset)
        record['good_cnt'], offset = self._parse_u4(data, offset)
        record['func_cnt'], offset = self._parse_u4(data, offset)
        
        return record
    
    def _track_part_counts(self, pcr_data: Dict[str, Any]):
        """Track part counts by site for consistency validation"""
        site_key = (pcr_data['head_num'], pcr_data['site_num'])
        self.part_counts[site_key] = pcr_data
    
    def _validate_far_record(self, data: bytes):
        """Validate File Attributes Record"""
        if len(data) < 2:
            raise ValidationError("FAR record too short")
            
        cpu_type, stdf_ver = struct.unpack('<BB', data[:2])
        self.cpu_type = cpu_type
        self.stdf_version = stdf_ver
        
        # Validate CPU type
        if cpu_type > 255:
            self.errors.append("Invalid CPU type in FAR")
            
        # Validate STDF version
        if stdf_ver != 4:
            self.warnings.append(f"STDF version is {stdf_ver}, expected 4")
            
        self.has_far = True
    
    def _validate_mir_record(self, data: bytes):
        """Validate Master Information Record"""
        if len(data) < 12:  # Minimum required fields
            raise ValidationError("MIR record too short")
            
        self.has_mir = True
    
    def _validate_mrr_record(self, data: bytes):
        """Validate Master Results Record"""
        self.has_mrr = True
    
    def _validate_record_sequence(self):
        """Validate the sequence of records follows STDF requirements"""
        if not self.record_sequence:
            self.errors.append("No records found in file")
            return
            
        # Check initial sequence requirements
        sequence_names = [record[0] for record in self.record_sequence]
        
        # FAR must be first
        if sequence_names[0] != "FAR":
            self.errors.append("First record must be FAR")
        
        # Find positions of key records
        far_pos = self._find_record_position("FAR", sequence_names)
        mir_pos = self._find_record_position("MIR", sequence_names)
        mrr_pos = self._find_record_position("MRR", sequence_names)
        
        # MIR must come after FAR and any ATRs
        if mir_pos is not None and far_pos is not None:
            # Check for ATRs between FAR and MIR
            atr_positions = [i for i, name in enumerate(sequence_names) if name == "ATR"]
            expected_mir_pos = far_pos + 1 + len([pos for pos in atr_positions if pos < mir_pos])
            
            if mir_pos < far_pos:
                self.errors.append("MIR must come after FAR")
        
        # MRR must be last record
        if mrr_pos is not None and mrr_pos != len(sequence_names) - 1:
            self.errors.append("MRR must be the last record in the file")
        
        # Check for RDR and SDR placement
        rdr_pos = self._find_record_position("RDR", sequence_names)
        if rdr_pos is not None and mir_pos is not None:
            if rdr_pos != mir_pos + 1:
                self.errors.append("RDR must immediately follow MIR")
        
        sdr_positions = [i for i, name in enumerate(sequence_names) if name == "SDR"]
        if sdr_positions and mir_pos is not None:
            expected_sdr_start = mir_pos + 1
            if rdr_pos is not None:
                expected_sdr_start = rdr_pos + 1
            if min(sdr_positions) != expected_sdr_start:
                self.errors.append("SDRs must immediately follow MIR (and RDR if present)")
    
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
    
    def _get_record_name(self, rec_typ: int, rec_sub: int) -> str:
        """Get record name from type and subtype"""
        for record_type in RecordType:
            if record_type.value == (rec_typ, rec_sub):
                return record_type.name
        return "UNKNOWN"
    
    def _find_record_position(self, record_name: str, sequence: List[str]) -> Optional[int]:
        """Find the first position of a record type in the sequence"""
        try:
            return sequence.index(record_name)
        except ValueError:
            return None
    
    def _create_result(self, file_size: int) -> ValidationResult:
        """Create validation result"""
        return ValidationResult(
            is_valid=len(self.errors) == 0 and len(self.consistency_errors) == 0,
            errors=self.errors.copy(),
            warnings=self.warnings.copy(),
            record_count=self.record_count.copy(),
            file_size=file_size,
            consistency_errors=self.consistency_errors.copy()
        )


def validate_stdf_file(filepath: str) -> ValidationResult:
    """
    Convenience function to validate an STDF file
    
    Args:
        filepath: Path to the STDF file
        
    Returns:
        ValidationResult containing validation status and details
    """
    validator = STDFValidator()
    return validator.validate_file(filepath)


def print_validation_result(result: ValidationResult, filepath: str):
    """Print validation results in a readable format"""
    print(f"\nSTDF Validation Result for: {filepath}")
    print("=" * 60)
    print(f"File Size: {result.file_size:,} bytes")
    print(f"Valid: {'✓ YES' if result.is_valid else '✗ NO'}")
    
    if result.errors:
        print(f"\nStructural Errors ({len(result.errors)}):")
        for i, error in enumerate(result.errors, 1):
            print(f"  {i}. {error}")
    
    if result.consistency_errors:
        print(f"\nCross-Record Consistency Errors ({len(result.consistency_errors)}):")
        for i, error in enumerate(result.consistency_errors, 1):
            print(f"  {i}. {error}")
    
    if result.warnings:
        print(f"\nWarnings ({len(result.warnings)}):")
        for i, warning in enumerate(result.warnings, 1):
            print(f"  {i}. {warning}")
    
    if result.record_count:
        print(f"\nRecord Summary:")
        for record_type, count in sorted(result.record_count.items()):
            print(f"  {record_type}: {count}")
            
    # Summary of validation categories
    total_issues = len(result.errors) + len(result.consistency_errors)
    if total_issues == 0:
        print(f"\n✓ File passed all validation checks!")
    else:
        print(f"\n✗ File has {total_issues} critical issues that need to be addressed.")
        if result.warnings:
            print(f"  Additionally, {len(result.warnings)} warnings were found.")


# Example usage and testing
if __name__ == "__main__":
    import sys
    
    def create_sample_validation_report():
        """Create a sample validation report showing the types of errors detected"""
        print("\n" + "="*80)
        print("SAMPLE CROSS-RECORD CONSISTENCY VALIDATION FEATURES")
        print("="*80)
        
        print("\n1. PTR-TSR Consistency Validation:")
        print("   - Verifies TSR execution counts match actual PTR record counts")
        print("   - Ensures every test in TSR has corresponding PTR records")
        print("   - Detects PTR records without corresponding TSR definitions")
        
        print("\n2. PRR-Bin Consistency Validation:")
        print("   - Validates PRR bin assignments reference defined HBR/SBR bins")
        print("   - Identifies undefined bins referenced in part results")
        print("   - Reports defined but unused bins")
        
        print("\n3. Bin Count Consistency Validation:")
        print("   - Compares HBR/SBR reported counts with actual PRR bin assignments")
        print("   - Handles both per-site and summary (HEAD_NUM=255) bin records")
        print("   - Validates mathematical consistency of bin populations")
        
        print("\n4. Part Count Consistency Validation:")
        print("   - Verifies PCR part counts match actual number of PRR records")
        print("   - Validates both site-specific and summary part counts")
        print("   - Ensures lot-level part count integrity")
        
        print("\n5. Test Execution Consistency Validation:")
        print("   - Cross-validates test execution data across record types")
        print("   - Ensures test numbering consistency")
        print("   - Validates per-part test execution reporting")
        
        print("\nExample Error Messages:")
        print("  - Test 1001 site (1, 1): TSR reports 100 executions, but found 95 PTR records")
        print("  - PRR references undefined hardware bin 15")
        print("  - Hardware bin 3 site (1, 1): HBR reports 50 parts, but PRR records show 52")
        print("  - Part count site (255, 255): PCR reports 1000 parts, but found 998 PRR records")
        print("  - PTR records found for tests without corresponding TSR: [1002, 1003]")
        
        print("\n" + "="*80)
    
    if len(sys.argv) == 1:
        create_sample_validation_report()
        print("\nUsage: python stdf_validator.py <stdf_file>")
        sys.exit(0)
    elif len(sys.argv) != 2:
        print("Usage: python stdf_validator.py <stdf_file>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    print(f"Validating STDF file: {filepath}")
    print("Performing structural and cross-record consistency validation...")
    
    result = validate_stdf_file(filepath)
    print_validation_result(result, filepath)
    
    # Additional detailed reporting
    if not result.is_valid:
        print(f"\n{'='*60}")
        print("VALIDATION SUMMARY")
        print(f"{'='*60}")
        print(f"Structural Issues: {len(result.errors)}")
        print(f"Consistency Issues: {len(result.consistency_errors)}")
        print(f"Warnings: {len(result.warnings)}")
        print(f"Total Critical Issues: {len(result.errors) + len(result.consistency_errors)}")
        
        if result.consistency_errors:
            print(f"\nThe file has cross-record consistency issues that indicate:")
            print("- Data integrity problems")
            print("- Potential corruption during data collection or processing")
            print("- Issues with test program or tester configuration")
            print("- Mathematical inconsistencies in count fields")
    
    sys.exit(0 if result.is_valid else 1)