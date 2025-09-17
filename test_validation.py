#!/usr/bin/env python3
"""
Simple test script to demonstrate the Python validation bindings.
"""

import stupidf as sf

def test_validation():
    """Test the STDF validation functionality."""
    
    try:
        # Test validation on the test STDF file
        print("Testing STDF validation...")
        report = sf.validate_stdf("test.stdf", strict=False)
        
        print(f"Validation Report:")
        print(f"  Valid: {report.is_valid}")
        print(f"  Total records: {report.total_records}")
        print(f"  Errors: {report.error_count_py}")
        print(f"  Warnings: {report.warning_count_py}")
        print(f"  Info: {report.info_count_py}")
        
        print(f"\nRecord type counts:")
        for record_type, count in report.record_type_counts.items():
            print(f"  {record_type}: {count}")
        
        if report.issues:
            print(f"\nIssues found:")
            for i, issue in enumerate(report.issues[:5]):  # Show first 5 issues
                print(f"  {i+1}. {issue.level}: {issue.message}")
                if issue.record_index is not None and issue.record_type is not None:
                    print(f"     At record #{issue.record_index} ({issue.record_type})")
        
        print("\nValidation test completed successfully!")
        
    except Exception as e:
        print(f"Validation test failed: {e}")
        return False
    
    return True

if __name__ == "__main__":
    test_validation()