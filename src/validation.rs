use std::collections::HashMap;
use crate::records::{Records, RawRecord};
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ValidationLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct ValidationIssue {
    #[pyo3(get)]
    pub level: ValidationLevel,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub record_index: Option<usize>,
    #[pyo3(get)]
    pub record_type: Option<String>,
}

impl ValidationIssue {
    pub fn error(message: String) -> Self {
        Self {
            level: ValidationLevel::Error,
            message,
            record_index: None,
            record_type: None,
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            level: ValidationLevel::Warning,
            message,
            record_index: None,
            record_type: None,
        }
    }

    pub fn info(message: String) -> Self {
        Self {
            level: ValidationLevel::Info,
            message,
            record_index: None,
            record_type: None,
        }
    }

    pub fn with_record_info(mut self, index: usize, record_type: String) -> Self {
        self.record_index = Some(index);
        self.record_type = Some(record_type);
        self
    }
}

#[derive(Debug)]
#[pyclass]
pub struct ValidationReport {
    #[pyo3(get)]
    pub issues: Vec<ValidationIssue>,
    #[pyo3(get)]
    pub is_valid: bool,
    #[pyo3(get)]
    pub total_records: usize,
    #[pyo3(get)]
    pub record_type_counts: HashMap<String, usize>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            is_valid: true,
            total_records: 0,
            record_type_counts: HashMap::new(),
        }
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        if matches!(issue.level, ValidationLevel::Error) {
            self.is_valid = false;
        }
        self.issues.push(issue);
    }

    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.level, ValidationLevel::Error)).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.level, ValidationLevel::Warning)).count()
    }

    pub fn info_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.level, ValidationLevel::Info)).count()
    }
}

#[pymethods]
impl ValidationReport {
    #[getter]
    fn error_count_py(&self) -> usize {
        self.error_count()
    }

    #[getter]
    fn warning_count_py(&self) -> usize {
        self.warning_count()
    }

    #[getter]
    fn info_count_py(&self) -> usize {
        self.info_count()
    }
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "STDF Validation Report")?;
        writeln!(f, "=====================")?;
        writeln!(f, "Status: {}", if self.is_valid { "VALID" } else { "INVALID" })?;
        writeln!(f, "Total Records: {}", self.total_records)?;
        writeln!(f, "Issues: {} errors, {} warnings, {} info", 
                 self.error_count(), self.warning_count(), self.info_count())?;
        writeln!(f)?;

        if !self.record_type_counts.is_empty() {
            writeln!(f, "Record Type Summary:")?;
            let mut types: Vec<_> = self.record_type_counts.iter().collect();
            types.sort_by_key(|(name, _)| *name);
            for (record_type, count) in types {
                writeln!(f, "  {}: {}", record_type, count)?;
            }
            writeln!(f)?;
        }

        if !self.issues.is_empty() {
            writeln!(f, "Issues Found:")?;
            for (i, issue) in self.issues.iter().enumerate() {
                let level_str = match issue.level {
                    ValidationLevel::Error => "ERROR",
                    ValidationLevel::Warning => "WARN",
                    ValidationLevel::Info => "INFO",
                };
                
                let location = if let (Some(idx), Some(rec_type)) = (&issue.record_index, &issue.record_type) {
                    format!(" [Record #{} ({})]", idx, rec_type)
                } else {
                    String::new()
                };
                
                writeln!(f, "  {}: {}{}: {}", i + 1, level_str, location, issue.message)?;
            }
        }

        Ok(())
    }
}

pub struct StdfValidator {
    strict_mode: bool,
}

impl StdfValidator {
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    pub fn validate(&self, filename: &str) -> Result<ValidationReport, Box<dyn std::error::Error>> {
        let mut report = ValidationReport::new();
        let records: Vec<RawRecord> = Records::new(filename)?.collect();
        
        report.total_records = records.len();
        
        self.count_record_types(&records, &mut report);
        self.validate_file_structure(&records, &mut report);
        self.validate_record_sequence(&records, &mut report);
        self.validate_data_consistency(&records, &mut report);
        
        Ok(report)
    }

    fn count_record_types(&self, records: &[RawRecord], report: &mut ValidationReport) {
        for record in records {
            let record_type = format!("{}:{}", record.header.rec_typ, record.header.rec_sub);
            *report.record_type_counts.entry(record_type).or_insert(0) += 1;
        }
    }

    fn validate_file_structure(&self, records: &[RawRecord], report: &mut ValidationReport) {
        if records.is_empty() {
            report.add_issue(ValidationIssue::error("File contains no records".to_string()));
            return;
        }

        let first_record = &records[0];
        if first_record.header.rec_typ != 0 || first_record.header.rec_sub != 10 {
            report.add_issue(ValidationIssue::error(
                "First record must be FAR (File Attributes Record)".to_string()
            ).with_record_info(0, "FAR".to_string()));
        }

        let has_mir = records.iter().any(|r| r.header.rec_typ == 1 && r.header.rec_sub == 10);
        if !has_mir {
            report.add_issue(ValidationIssue::error(
                "File must contain at least one MIR (Master Information Record)".to_string()
            ));
        }

        let has_mrr = records.iter().any(|r| r.header.rec_typ == 1 && r.header.rec_sub == 20);
        if !has_mrr {
            report.add_issue(ValidationIssue::error(
                "File must contain at least one MRR (Master Results Record)".to_string()
            ));
        }

        let has_pcr = records.iter().any(|r| r.header.rec_typ == 1 && r.header.rec_sub == 30);
        if !has_pcr {
            report.add_issue(ValidationIssue::error(
                "File must contain at least one PCR (Part Count Record)".to_string()
            ));
        }
    }

    fn validate_record_sequence(&self, records: &[RawRecord], report: &mut ValidationReport) {
        let mut found_far = false;
        let mut found_mir = false;
        let mut found_mrr = false;
        let mut initial_sequence_complete = false;

        for (i, record) in records.iter().enumerate() {
            let rec_typ = record.header.rec_typ;
            let rec_sub = record.header.rec_sub;
            let record_name = self.get_record_name(rec_typ, rec_sub);

            match (rec_typ, rec_sub) {
                (0, 10) => { // FAR
                    if i != 0 {
                        report.add_issue(ValidationIssue::error(
                            "FAR must be the first record in the file".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                    found_far = true;
                },
                (0, 20) => { // ATR
                    if !found_far {
                        report.add_issue(ValidationIssue::error(
                            "ATR must appear after FAR".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                    if found_mir {
                        report.add_issue(ValidationIssue::warning(
                            "ATR should appear before MIR".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                },
                (1, 10) => { // MIR
                    if !found_far {
                        report.add_issue(ValidationIssue::error(
                            "MIR must appear after FAR".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                    found_mir = true;
                },
                (1, 20) => { // MRR
                    found_mrr = true;
                    initial_sequence_complete = true;
                },
                (1, 70) => { // RDR
                    if !found_mir {
                        report.add_issue(ValidationIssue::error(
                            "RDR must appear after MIR".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                },
                (1, 80) => { // SDR
                    if !found_mir {
                        report.add_issue(ValidationIssue::error(
                            "SDR must appear after MIR".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                },
                (5, 10) => { // PIR
                    if !initial_sequence_complete && !found_mir {
                        report.add_issue(ValidationIssue::error(
                            "PIR must appear after initial sequence".to_string()
                        ).with_record_info(i, record_name.clone()));
                    }
                },
                _ => {}
            }
        }
    }

    fn validate_data_consistency(&self, records: &[RawRecord], report: &mut ValidationReport) {
        let mut pir_count = 0;
        let mut prr_count = 0;
        let mut test_records_without_pir = 0;
        let mut current_part_active = false;

        for (i, record) in records.iter().enumerate() {
            let rec_typ = record.header.rec_typ;
            let rec_sub = record.header.rec_sub;
            let record_name = self.get_record_name(rec_typ, rec_sub);

            match (rec_typ, rec_sub) {
                (5, 10) => { // PIR
                    pir_count += 1;
                    current_part_active = true;
                },
                (5, 20) => { // PRR
                    prr_count += 1;
                    current_part_active = false;
                },
                (15, 10) | (15, 15) | (15, 20) => { // PTR, MPR, FTR
                    if !current_part_active {
                        test_records_without_pir += 1;
                        if self.strict_mode {
                            report.add_issue(ValidationIssue::error(
                                "Test record found without active PIR".to_string()
                            ).with_record_info(i, record_name.clone()));
                        }
                    }
                },
                _ => {}
            }

            if record.header.rec_len as usize != record.contents.len() {
                report.add_issue(ValidationIssue::error(
                    format!("Record length mismatch: header says {}, actual content is {} bytes", 
                           record.header.rec_len, record.contents.len())
                ).with_record_info(i, record_name));
            }
        }

        if pir_count != prr_count {
            report.add_issue(ValidationIssue::warning(
                format!("PIR/PRR count mismatch: {} PIR records, {} PRR records", pir_count, prr_count)
            ));
        }

        if test_records_without_pir > 0 && !self.strict_mode {
            report.add_issue(ValidationIssue::warning(
                format!("{} test records found without active PIR", test_records_without_pir)
            ));
        }
    }

    fn get_record_name(&self, rec_typ: u8, rec_sub: u8) -> String {
        match (rec_typ, rec_sub) {
            (0, 10) => "FAR".to_string(),
            (0, 20) => "ATR".to_string(),
            (1, 10) => "MIR".to_string(),
            (1, 20) => "MRR".to_string(),
            (1, 30) => "PCR".to_string(),
            (1, 40) => "HBR".to_string(),
            (1, 50) => "SBR".to_string(),
            (1, 60) => "PMR".to_string(),
            (1, 62) => "PGR".to_string(),
            (1, 63) => "PLR".to_string(),
            (1, 70) => "RDR".to_string(),
            (1, 80) => "SDR".to_string(),
            (2, 10) => "WIR".to_string(),
            (2, 20) => "WRR".to_string(),
            (2, 30) => "WCR".to_string(),
            (5, 10) => "PIR".to_string(),
            (5, 20) => "PRR".to_string(),
            (10, 30) => "TSR".to_string(),
            (15, 10) => "PTR".to_string(),
            (15, 15) => "MPR".to_string(),
            (15, 20) => "FTR".to_string(),
            (20, 10) => "BPS".to_string(),
            (20, 20) => "EPS".to_string(),
            (50, 10) => "GDR".to_string(),
            (50, 30) => "DTR".to_string(),
            _ => format!("{}:{}", rec_typ, rec_sub),
        }
    }
}

pub fn validate_stdf_file(filename: &str, strict_mode: bool) -> Result<ValidationReport, Box<dyn std::error::Error>> {
    let validator = StdfValidator::new(strict_mode);
    validator.validate(filename)
}