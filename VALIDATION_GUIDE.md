# STDF Validation Guide

This guide explains how to use the STDF validation functionality in both Rust CLI and Python.

## Overview

The STDF validation system provides comprehensive health checking for STDF files to ensure they comply with the STDF V4 specification and identify potential data integrity issues.

## Features

### Validation Checks

1. **File Structure Validation**
   - Verifies presence of required records (FAR, MIR, MRR, PCR)
   - Ensures first record is FAR (File Attributes Record)
   - Checks for mandatory STDF file components

2. **Record Sequence Validation**
   - Validates proper STDF record ordering per specification
   - Ensures FAR appears first
   - Checks that ATR records appear after FAR and before MIR
   - Validates initial sequence compliance

3. **Data Consistency Validation**
   - Checks PIR/PRR (Part Information/Results) pairing
   - Validates record length consistency
   - Ensures test records have corresponding part records
   - Detects orphaned test data

4. **Record Statistics**
   - Counts all record types in the file
   - Provides comprehensive file statistics
   - Identifies record distribution patterns

### Validation Modes

- **Standard Mode**: Basic validation checks suitable for most use cases
- **Strict Mode**: Enhanced validation with stricter rules for production environments

## CLI Usage

### Basic Validation

```bash
# Validate an STDF file with standard checks
cargo run -- --validate test.stdf

# Or use the built binary
./stupidf --validate test.stdf
```

### Strict Validation

```bash
# Validate with strict mode enabled
cargo run -- --validate --strict test.stdf

# Built binary with strict mode
./stupidf --validate --strict test.stdf
```

### Example CLI Output

```
STDF Validation Report
=====================
Status: VALID
Total Records: 1247
Issues: 0 errors, 2 warnings, 1 info

Record Type Summary:
  0:10: 1      # FAR (File Attributes Record)
  1:10: 1      # MIR (Master Information Record)
  1:20: 1      # MRR (Master Results Record)  
  1:30: 4      # PCR (Part Count Record)
  5:10: 100    # PIR (Part Information Record)
  5:20: 100    # PRR (Part Results Record)
  15:10: 1040  # PTR (Parametric Test Record)

Issues Found:
  1: WARN [Record #156 (ATR)]: ATR should appear before MIR
  2: WARN: PIR/PRR count mismatch: 100 PIR records, 99 PRR records
  3: INFO: Test record validation completed
```

## Python API

### Installation

First, build the Python bindings:

```bash
pip install maturin
maturin develop
```

### Basic Usage

```python
import stupidf as sf

# Validate an STDF file
report = sf.validate_stdf("test.stdf")

# Check if file is valid
if report.is_valid:
    print("✓ STDF file is valid!")
else:
    print(f"✗ Found {report.error_count_py} errors")

# Show basic statistics
print(f"Total records: {report.total_records}")
print(f"Errors: {report.error_count_py}")
print(f"Warnings: {report.warning_count_py}")
print(f"Info: {report.info_count_py}")
```

### Strict Validation

```python
import stupidf as sf

# Enable strict validation mode
report = sf.validate_stdf("test.stdf", strict=True)

if not report.is_valid:
    print("Validation failed in strict mode")
    for issue in report.issues:
        if issue.level == sf.ValidationLevel.Error:
            print(f"ERROR: {issue.message}")
```

### Detailed Issue Analysis

```python
import stupidf as sf

report = sf.validate_stdf("test.stdf")

# Examine all issues
for i, issue in enumerate(report.issues):
    print(f"Issue {i+1}:")
    print(f"  Level: {issue.level}")
    print(f"  Message: {issue.message}")
    
    # Show location if available
    if issue.record_index is not None and issue.record_type is not None:
        print(f"  Location: Record #{issue.record_index} ({issue.record_type})")
    print()
```

### Record Type Analysis

```python
import stupidf as sf

report = sf.validate_stdf("test.stdf")

print("Record Type Distribution:")
for record_type, count in sorted(report.record_type_counts.items()):
    print(f"  {record_type}: {count}")

# Find most common record types
sorted_records = sorted(report.record_type_counts.items(), 
                       key=lambda x: x[1], reverse=True)
print(f"\nMost common record: {sorted_records[0][0]} ({sorted_records[0][1]} occurrences)")
```

### Validation Workflow Integration

```python
import stupidf as sf

def process_stdf_file(filename):
    """Process STDF file with validation checks."""
    
    # First, validate the file
    print(f"Validating {filename}...")
    report = sf.validate_stdf(filename, strict=False)
    
    if not report.is_valid:
        print(f"❌ Validation failed with {report.error_count_py} errors:")
        for issue in report.issues:
            if issue.level == sf.ValidationLevel.Error:
                print(f"  - {issue.message}")
        return None
    
    # Show warnings but continue processing
    if report.warning_count_py > 0:
        print(f"⚠️  Found {report.warning_count_py} warnings:")
        for issue in report.issues:
            if issue.level == sf.ValidationLevel.Warning:
                print(f"  - {issue.message}")
    
    print("✅ Validation passed, proceeding with parsing...")
    
    # Now parse the validated file
    stdf_data = sf.parse_stdf(filename)
    return stdf_data

# Example usage
data = process_stdf_file("my_file.stdf")
if data:
    print(f"Successfully processed file with {len(data['df'])} test results")
```

## ValidationReport API Reference

### Properties

- `is_valid: bool` - Whether the file passed validation
- `total_records: int` - Total number of records in the file
- `issues: List[ValidationIssue]` - List of all validation issues found
- `record_type_counts: Dict[str, int]` - Count of each record type

### Methods

- `error_count_py: int` - Number of error-level issues
- `warning_count_py: int` - Number of warning-level issues  
- `info_count_py: int` - Number of info-level issues

## ValidationIssue API Reference

### Properties

- `level: ValidationLevel` - Severity level (Error, Warning, Info)
- `message: str` - Description of the issue
- `record_index: Optional[int]` - Index of the problematic record (if applicable)
- `record_type: Optional[str]` - Type of the problematic record (if applicable)

## ValidationLevel Enum

- `ValidationLevel.Error` - Critical issues that make the file invalid
- `ValidationLevel.Warning` - Issues that should be reviewed but don't invalidate the file
- `ValidationLevel.Info` - Informational messages about the validation process

## Best Practices

### 1. Always Validate Before Processing

```python
# Good practice
report = sf.validate_stdf("file.stdf")
if report.is_valid:
    data = sf.parse_stdf("file.stdf")
    # Process data...
else:
    # Handle validation errors
    print("File validation failed")
```

### 2. Use Strict Mode for Production

```python
# For production systems
report = sf.validate_stdf("file.stdf", strict=True)
if not report.is_valid:
    raise ValueError(f"STDF file failed strict validation: {report.error_count_py} errors")
```

### 3. Log Validation Results

```python
import logging

def validate_and_log(filename):
    report = sf.validate_stdf(filename)
    
    if report.is_valid:
        logging.info(f"✓ {filename} validation passed ({report.total_records} records)")
    else:
        logging.error(f"✗ {filename} validation failed ({report.error_count_py} errors)")
        
    for issue in report.issues:
        level = logging.ERROR if issue.level == sf.ValidationLevel.Error else logging.WARNING
        logging.log(level, f"{filename}: {issue.message}")
    
    return report.is_valid
```

### 4. Batch Validation

```python
import os

def validate_directory(directory):
    """Validate all STDF files in a directory."""
    results = {}
    
    for filename in os.listdir(directory):
        if filename.endswith('.stdf'):
            filepath = os.path.join(directory, filename)
            try:
                report = sf.validate_stdf(filepath)
                results[filename] = {
                    'valid': report.is_valid,
                    'errors': report.error_count_py,
                    'warnings': report.warning_count_py,
                    'records': report.total_records
                }
            except Exception as e:
                results[filename] = {'error': str(e)}
    
    return results

# Usage
results = validate_directory("./stdf_files/")
for filename, result in results.items():
    if 'error' in result:
        print(f"❌ {filename}: {result['error']}")
    elif result['valid']:
        print(f"✅ {filename}: {result['records']} records")
    else:
        print(f"⚠️  {filename}: {result['errors']} errors, {result['warnings']} warnings")
```

## Troubleshooting

### Common Validation Errors

1. **"First record must be FAR"**
   - File doesn't start with File Attributes Record
   - File may be corrupted or not a valid STDF file

2. **"File must contain at least one MIR"**
   - Missing Master Information Record
   - File may be incomplete or corrupted

3. **"PIR/PRR count mismatch"**
   - Unmatched Part Information and Part Results records
   - May indicate incomplete test data

4. **"Record length mismatch"**
   - Record header length doesn't match actual content
   - File corruption or parsing issue

### Performance Considerations

- Validation is generally fast but scales with file size
- For very large files (>100MB), consider validating a sample first
- Use strict mode only when necessary as it performs additional checks

### Error Handling

```python
import stupidf as sf

try:
    report = sf.validate_stdf("file.stdf")
except FileNotFoundError:
    print("STDF file not found")
except PermissionError:
    print("Permission denied accessing STDF file")
except Exception as e:
    print(f"Unexpected validation error: {e}")
```

## Integration Examples

### With pytest

```python
import pytest
import stupidf as sf

def test_stdf_file_validation():
    """Test that STDF file is valid."""
    report = sf.validate_stdf("test_data.stdf")
    
    assert report.is_valid, f"STDF validation failed with {report.error_count_py} errors"
    assert report.total_records > 0, "File contains no records"
    assert report.error_count_py == 0, "File contains validation errors"

def test_stdf_file_structure():
    """Test specific STDF file structure requirements."""
    report = sf.validate_stdf("test_data.stdf")
    
    # Check for required record types
    assert "0:10" in report.record_type_counts, "Missing FAR record"
    assert "1:10" in report.record_type_counts, "Missing MIR record"
    assert "1:20" in report.record_type_counts, "Missing MRR record"
```

### With pandas for analysis

```python
import stupidf as sf
import pandas as pd

def analyze_stdf_files(file_list):
    """Analyze multiple STDF files and create summary report."""
    results = []
    
    for filename in file_list:
        report = sf.validate_stdf(filename)
        results.append({
            'filename': filename,
            'valid': report.is_valid,
            'total_records': report.total_records,
            'errors': report.error_count_py,
            'warnings': report.warning_count_py,
            'ptr_count': report.record_type_counts.get('15:10', 0),  # PTR records
            'ftr_count': report.record_type_counts.get('15:20', 0),  # FTR records
        })
    
    df = pd.DataFrame(results)
    return df

# Create analysis report
files = ['file1.stdf', 'file2.stdf', 'file3.stdf']
summary = analyze_stdf_files(files)
print(summary.describe())
```

This comprehensive guide should help you effectively use the STDF validation functionality in your projects!