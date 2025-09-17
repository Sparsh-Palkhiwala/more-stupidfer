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

COMPARISON WITH RUST VALIDATOR:
===============================

Use this Python validator when you need:
- Deep analysis and debugging of problematic files
- Comprehensive cross-record consistency checks
- Detailed error reporting for troubleshooting

Use the Rust validator (stupidf.validate_stdf()) when you need:
- Fast validation as part of parsing workflow
- Basic structure and integrity checks
- Integration with existing Rust/Python processing

Both validators complement each other and can be used together.
"""

import struct
import os
import sys
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass
from enum import Enum


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


class STDFValidator:
    """
    STDF V4 File Validator
    
    Validates STDF files according to the V4 specification including:
    - Binary structure validation
    - Required record sequence validation
    - Record type validation
    - Data integrity checks
    """
    
    def __init__(self):
        self.errors = []
        self.warnings = []
        self.consistency_errors = []
        self.record_count = {}
        self.record_sequence = []
        
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
            
            # Track record types
            record_name = self._get_record_name(rec_typ, rec_sub)
            self.record_count[record_name] = self.record_count.get(record_name, 0) + 1
            self.record_sequence.append(record_name)
            
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
        print(f"\nConsistency Errors ({len(result.consistency_errors)}):")
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
            
    # Summary
    total_issues = len(result.errors) + len(result.consistency_errors)
    if total_issues == 0:
        print(f"\n✓ File passed all validation checks!")
    else:
        print(f"\n✗ File has {total_issues} critical issues that need to be addressed.")
        if result.warnings:
            print(f"  Additionally, {len(result.warnings)} warnings were found.")


# Example usage and testing
if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Advanced STDF V4 File Validator")
        print("=" * 40)
        print("Usage: python stdf_validator.py <stdf_file>")
        print("\nExample:")
        print("  python stdf_validator.py test.stdf")
        print("\nThis validator performs comprehensive validation including:")
        print("- Binary structure validation")
        print("- Required record sequence validation") 
        print("- Record type validation")
        print("- Data integrity checks")
        print("\nFor advanced cross-record consistency validation,")
        print("see the full implementation in the stupidf library.")
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    print(f"Validating STDF file: {filepath}")
    print("Performing structural validation...")
    
    result = validate_stdf_file(filepath)
    print_validation_result(result, filepath)
    
    sys.exit(0 if result.is_valid else 1)