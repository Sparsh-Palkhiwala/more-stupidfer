use crate::records::RecordSummary;
use crate::records::Records;
use crate::records::records::PTR;
use crate::records::records::Record;
use crate::records::records::TSR;
use polars::frame::DataFrame;
use polars::prelude::Column;
use pyo3::Bound;
use pyo3::IntoPyObject;
use pyo3::Python;
use pyo3::types::PyString;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;

/// `TestInformation` for a single test
///
/// The metadata for a test is uniquely determined by (`test_num`, `head_num`, `site_num`). This
/// structure does not contain information about any executions of the test, it only contains
/// metadata associated with the test.
#[derive(Debug, IntoPyObject)]
pub struct TestInformation {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_type: TestType,
    pub execution_count: u32,
    pub test_name: String,
    pub sequence_name: String,
    pub test_label: String,
    pub test_time: f32,
    pub test_text: String,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub res_scal: i8,
    pub lo_spec: f32,
    pub hi_spec: f32,
    pub low_limit: f32,
    pub high_limit: f32,
    pub units: String,
    pub complete: Complete,

    pub res_scal: Option<i8>,
    pub llm_scal: Option<i8>,
    pub hlm_scal: Option<i8>,
}

/// Enum describing if a `TestInformation` has been completed
///
/// `TestInformation` is determined by the combination of a `TSR` and at least one `PTR`.
/// The `TSR` variant indicates that the metadata from a `TSR` has been added.
/// The `PTR` variant indicates that the metadata from a `PTR` has been added.
/// The `Complete` variant indicates that both a `TSR` and `PTR` have been seen.
#[derive(Debug)]
pub enum Complete {
    /// Metadata from a PTR has been added to the owning TestInformation
    PTR,
    /// Metadata from a TSR has been added to the owning TestInformation
    TSR,
    /// Metadata from both a PTR and TSR have been added to the owning TestInformation
    Complete,
}

/// Determines how to convert `Complete` into Python objects
///
/// Can't derive `IntoPyObject` for enums, so implement manually.
/// The variants are simply converted to strings of their variant names
impl<'py> IntoPyObject<'py> for Complete {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::TSR => "TSR".to_string().into_pyobject(py),
            Self::PTR => "PTR".to_string().into_pyobject(py),
            Self::Complete => "Complete".to_string().into_pyobject(py),
        };
        Ok(s?)
    }
}

impl TestInformation {
    /// Create a new `TestInformation` from a `PTR` record
    pub fn new_from_ptr(ptr: &PTR) -> Self {
        let test_num = ptr.test_num;
        let head_num = ptr.head_num;
        let site_num = ptr.site_num;
        let test_type = TestType::Unknown;
        let execution_count = 0;
        let test_name = String::new();
        let sequence_name = String::new();
        let test_label = String::new();
        let test_time = f32::NAN;
        let llm_scal = ptr.llm_scal;
        let hlm_scal = ptr.hlm_scal;
        let res_scal = ptr.res_scal;
        let lo_spec = ptr.lo_spec;
        let hi_spec = ptr.hi_spec;
        let test_text = ptr.test_txt.clone();
        let low_limit = ptr.lo_limit;
        let high_limit = ptr.hi_limit;
        let units = ptr.units.clone();
        let complete = Complete::PTR;
        let res_scal = Some(ptr.res_scal);
        let llm_scal = Some(ptr.llm_scal);
        let hlm_scal = Some(ptr.hlm_scal);

        Self {
            test_num,
            head_num,
            site_num,
            test_type,
            execution_count,
            test_name,
            sequence_name,
            test_label,
            test_time,
            test_text,
            llm_scal,
            hlm_scal,
            res_scal,
            lo_spec,
            hi_spec,
            low_limit,
            high_limit,
            units,
            complete,
            res_scal,
            llm_scal,
            hlm_scal,
        }
    }

    /// Add to an existing `TestInformation` with a `TSR` record
    pub fn add_from_tsr(&mut self, tsr: &TSR) {
        if (self.head_num != tsr.head_num)
            || (self.site_num != tsr.site_num)
            || (self.test_num != tsr.test_num)
        {
            panic!("head_num/site_num/test_num from TSR does not match!");
        }
        if let Complete::PTR = self.complete {
            self.test_type = match tsr.test_typ {
                'P' => TestType::P,
                'F' => TestType::F,
                'M' => TestType::M,
                'S' => TestType::S,
                _ => TestType::Unknown,
            };
            self.execution_count = tsr.exec_cnt;
            self.test_name = tsr.test_nam.clone();
            self.sequence_name = tsr.seq_name.clone();
            self.test_label = tsr.test_lbl.clone();
            self.test_time = tsr.test_tim;
            self.complete = Complete::Complete;
        }
    }

    /// Create a new `TestInformation` from a `TSR` record
    pub fn new_from_tsr(tsr: &TSR) -> Self {
        let test_num = tsr.test_num;
        let head_num = tsr.head_num;
        let site_num = tsr.site_num;
        let test_type = match tsr.test_typ {
            'P' => TestType::P,
            'F' => TestType::F,
            'M' => TestType::M,
            'S' => TestType::S,
            _ => TestType::Unknown,
        };
        let execution_count = tsr.exec_cnt;
        let test_name = tsr.test_nam.clone();
        let sequence_name = tsr.seq_name.clone();
        let test_label = tsr.test_lbl.clone();
        let test_time = tsr.test_tim;
        let test_text = String::new();
        let llm_scal = 0i8;  // Default value for i8 since TSR doesn't have this field
        let hlm_scal = 0i8;  // Default value for i8 since TSR doesn't have this field
        let res_scal = 0i8;  // Default value for i8 since TSR doesn't have this field
        let lo_spec = f32::NAN;
        let hi_spec = f32::NAN;
        let low_limit = f32::NAN;
        let high_limit = f32::NAN;
        let units = String::new();
        let complete = Complete::TSR;
        let res_scal = None;
        let llm_scal = None;
        let hlm_scal = None;
        Self {
            test_num,
            head_num,
            site_num,
            test_type,
            execution_count,
            test_name,
            sequence_name,
            test_label,
            test_time,
            test_text,
            llm_scal,
            hlm_scal,
            res_scal,
            lo_spec,
            hi_spec,
            low_limit,
            high_limit,
            units,
            complete,
            res_scal,
            llm_scal,
            hlm_scal,
        }
    }

    /// Add to an existing `TestInformation` with a `PTR`
    pub fn add_from_ptr(&mut self, ptr: &PTR) {
        if (self.head_num != ptr.head_num)
            || (self.site_num != ptr.site_num)
            || (self.test_num != ptr.test_num)
        {
            panic!("head_num/site_num/test_num from PTR does not match!");
        }
        if let Complete::TSR = self.complete {
            self.llm_scal = ptr.llm_scal;
            self.hlm_scal = ptr.hlm_scal;
            self.res_scal = ptr.res_scal;
            self.lo_spec = ptr.lo_spec;
            self.hi_spec = ptr.hi_spec;
            self.test_text = ptr.test_txt.clone();
            self.low_limit = ptr.lo_limit;
            self.high_limit = ptr.hi_limit;
            self.units = ptr.units.clone();
            self.complete = Complete::Complete;
        }
    }
}

/// `TestType` describes the category of test
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize)]
pub enum TestType {
    /// A parametric test, i.e. one that measures a value
    P,
    /// A functional test, i.e. one with only pass/fail
    F,
    /// A multi-result parametric test, i.e. one that measures many values
    M,
    /// A scan test
    S,
    /// An unknown test type
    Unknown,
}

impl fmt::Display for TestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Determines how to convert `TestType` into Python objects
///
/// Can't derive `IntoPyObject` for enums, so implement manually.
/// The variants are simply converted to strings of their variant names
impl<'py> IntoPyObject<'py> for TestType {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::P => "P".to_string().into_pyobject(py),
            Self::F => "F".to_string().into_pyobject(py),
            Self::M => "M".to_string().into_pyobject(py),
            Self::S => "S".to_string().into_pyobject(py),
            Self::Unknown => "Unknown".to_string().into_pyobject(py),
        };
        Ok(s?)
    }
}

/// A collection of all `TestInformation`s in a STDF file
///
/// Indexed by (`test_num`, `site_num`, `head_num`)
#[derive(Debug, IntoPyObject)]
pub struct FullTestInformation {
    pub test_infos: HashMap<(u32, u8, u8), TestInformation>,
}

impl FullTestInformation {
    /// Initialize with an empty HashMap
    pub fn new() -> Self {
        let test_infos = HashMap::new();
        Self { test_infos }
    }

    /// Add the metadata from a `PTR`.
    ///
    /// Looks up the appropriate `TestInformation` using the (`test_num`, `site_num`, `head_num`)
    /// in the `PTR` and adds to this `TestInformation`
    pub fn add_from_ptr(&mut self, ptr: &PTR) {
        let key = (ptr.test_num, ptr.site_num, ptr.head_num);
        self.test_infos
            .entry(key)
            .and_modify(|e| e.add_from_ptr(ptr))
            .or_insert(TestInformation::new_from_ptr(ptr));
    }

    /// Add the metadata from a `TSR`.
    ///
    /// Looks up the appropriate `TestInformation` using the (`test_num`, `site_num`, `head_num`)
    /// in the `TSR` and adds to this `TestInformation`
    pub fn add_from_tsr(&mut self, tsr: &TSR) {
        if tsr.head_num == 255 {
            return;
        }
        let key = (tsr.test_num, tsr.site_num, tsr.head_num);
        self.test_infos
            .entry(key)
            .and_modify(|e| e.add_from_tsr(tsr))
            .or_insert(TestInformation::new_from_tsr(tsr));
    }

    /// Merges down the `FullTestInformation` to a `FullMergedTestInformation`
    ///
    /// The `FullMergedTestInformation` is indexed by only `test_num` rather than
    /// (`test_num`, `site_num`, `head_num`). Usually all sites and all heads implement the
    /// same tests and just run them in parallel, so it's not necessary to keep track of the
    /// `site_num` and `head_num`.
    pub fn merge(&self) -> FullMergedTestInformation {
        let mut merged_test_info = FullMergedTestInformation::new();
        for ti in self.test_infos.values() {
            merged_test_info.add_from_test_information(ti);
        }
        merged_test_info
    }

    /// Gather all of the test information from a STDF specified by `fname`
    ///
    /// Iterates over all records in the STDF.
    ///
    /// Optionally allows for printing of the records while iterating over them, e.g. if you want
    /// to make an ASCII text version.
    ///
    /// TODO: Make which records are printed configurable
    ///
    /// # Errors
    /// If for some reason the file can't be parsed, returns a std::io::Error
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let records = Records::new(&fname)?;
        let mut test_info = Self::new();

        for record in records {
            if let Some(resolved) = record.resolve() {
                let header = &record.header;

                if verbose {
                    println!(
                        "{}.{} (0x{:x} @ 0x{:x}): {:?}",
                        header.rec_typ, header.rec_sub, header.rec_len, record.offset, record.rtype
                    );
                }
                if let Record::TSR(ref tsr) = resolved {
                    test_info.add_from_tsr(&tsr);
                }
                if let Record::PIR(_) = resolved {
                    continue;
                }
                if let Record::FTR(_) = resolved {
                    continue;
                }
                if let Record::PTR(ref ptr) = resolved {
                    test_info.add_from_ptr(&ptr);
                }
                //if let Record::PRR(_) = resolved {
                //    continue;
                //}
                if verbose {
                    println!("{resolved:#?}");
                }
            }
        }
        Ok(test_info)
    }

    pub fn from_fname_and_summarize(
        fname: &str,
        verbose: bool,
    ) -> std::io::Result<(Self, RecordSummary)> {
        let records = Records::new(&fname)?;
        let mut summary = RecordSummary::new();
        let mut test_info = Self::new();

        for record in records {
            summary.add(&record);
            if let Some(resolved) = record.resolve() {
                let header = &record.header;

                if verbose {
                    println!(
                        "{}.{} (0x{:x} @ 0x{:x}): {:?}",
                        header.rec_typ, header.rec_sub, header.rec_len, record.offset, record.rtype
                    );
                }
                if let Record::TSR(ref tsr) = resolved {
                    test_info.add_from_tsr(&tsr);
                }
                if let Record::PIR(_) = resolved {
                    continue;
                }
                if let Record::FTR(_) = resolved {
                    continue;
                }
                if let Record::PTR(ref ptr) = resolved {
                    test_info.add_from_ptr(&ptr);
                }
                //if let Record::PRR(_) = resolved {
                //    continue;
                //}
                if verbose {
                    println!("{resolved:#?}");
                }
            }
        }
        Ok((test_info, summary))
    }
}

impl IntoIterator for FullTestInformation {
    type Item = ((u32, u8, u8), TestInformation);
    type IntoIter = <HashMap<(u32, u8, u8), TestInformation> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.test_infos.into_iter()
    }
}

/// `MergedTestInformation` for a single test
///
/// The metadata for a test is uniquely determined by (`test_num`, `head_num`, `site_num`),
/// however typically the `head_num` and `site_num` are redundant since all heads and sites are
/// measuring the same thing in parallel. Given that, the `MergedTestInformation` contains the
/// `TestInformation` associated with all `head_num`s and `site_num`s for a given `test_num`
/// merged down to one structure.
///
/// The first (`test_num`, `head_num`, `site_num`) for a given `test_num` determines all of the
/// metadata. Subsequent triplets with the same `test_num` simply add to the `execution_count`.
///
/// This structure does not contain information about any executions of the test, it only contains
/// metadata associated with the test.
#[derive(Debug, IntoPyObject)]
pub struct MergedTestInformation {
    pub test_num: u32,
    pub test_type: TestType,
    pub execution_count: u32,
    pub test_name: String,
    pub sequence_name: String,
    pub test_label: String,
    pub test_time: f32,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub res_scal: i8,
    pub lo_spec: f32,
    pub hi_spec: f32,
    pub test_text: String,
    pub low_limit: f32,
    pub high_limit: f32,
    pub units: String,
}
impl MergedTestInformation {
    /// Initialize a new `MergedTestInformation` from a `TestInformation` record
    pub fn new_from_test_information(test_information: &TestInformation) -> Self {
        let test_num = test_information.test_num;
        let test_type = test_information.test_type.clone();
        let execution_count = test_information.execution_count;
        let test_name = test_information.test_name.clone();
        let sequence_name = test_information.sequence_name.clone();
        let test_label = test_information.test_label.clone();
        let test_time = test_information.test_time;
        let llm_scal = test_information.llm_scal;
        let hlm_scal = test_information.hlm_scal;
        let res_scal = test_information.res_scal;
        let lo_spec = test_information.lo_spec;
        let hi_spec = test_information.hi_spec;
        let test_text = test_information.test_text.clone();
        let low_limit = test_information.low_limit;
        let high_limit = test_information.high_limit;
        let units = test_information.units.clone();
        Self {
            test_num,
            test_type,
            execution_count,
            test_name,
            sequence_name,
            test_label,
            test_time,
            test_text,
            llm_scal,
            hlm_scal,
            res_scal,
            lo_spec,
            hi_spec,
            low_limit,
            high_limit,
            units,
        }
    }

    /// Add the `execution_count` from a `TestInformation` to an existing `MergedTestInformation`
    pub fn add(&mut self, test_information: &TestInformation) {
        if self.test_num != test_information.test_num {
            panic!("TestInformation.test_num does not match that of MergedTestInformation!")
        }
        self.execution_count += test_information.execution_count;
    }
}

/// A collection of all `MergedTestInformation`s in a STDF file
///
/// Indexed by `test_num`
#[derive(Debug, IntoPyObject)]
pub struct FullMergedTestInformation {
    pub test_infos: HashMap<u32, MergedTestInformation>,
}
impl FullMergedTestInformation {
    /// Initialize a new `FullMergedTestInformation` with an empty HashMap
    pub fn new() -> Self {
        let test_infos = HashMap::new();
        Self { test_infos }
    }

    /// Adds the metadata from a `TestInformation` record
    ///
    /// If there is not a corresponding `MergedTestInformation` for the `test_num`, a new one is
    /// made. If there is already a corresponding `MergedTestInformation`, adds the
    /// `execution_count`.
    pub fn add_from_test_information(&mut self, test_information: &TestInformation) {
        let key = test_information.test_num;
        self.test_infos
            .entry(key)
            .and_modify(|e| e.add(test_information))
            .or_insert(MergedTestInformation::new_from_test_information(
                test_information,
            ));
    }

    /// Get the number of different tests with a given `TestType`
    pub fn get_num(&self, test_type: TestType) -> usize {
        self.test_infos
            .values()
            .filter(|&mti| mti.test_type == test_type)
            .collect::<Vec<_>>()
            .len()
    }
}

/// Make a DataFrame containing the info in a `FullMergedTestInformation`
impl Into<DataFrame> for &FullMergedTestInformation {
    fn into(self) -> DataFrame {
        let mut test_nums: Vec<u32> = Vec::new();
        let mut test_types: Vec<String> = Vec::new();
        let mut execution_counts: Vec<u32> = Vec::new();
        let mut test_names: Vec<String> = Vec::new();
        let mut sequence_names: Vec<String> = Vec::new();
        let mut test_labels: Vec<String> = Vec::new();
        let mut test_times: Vec<f32> = Vec::new();
        let mut test_texts: Vec<String> = Vec::new();
        let mut llm_scals: Vec<i32> = Vec::new();
        let mut hlm_scals: Vec<i32> = Vec::new();
        let mut res_scals: Vec<i32> = Vec::new();
        let mut lo_specs: Vec<f32> = Vec::new();
        let mut hi_specs: Vec<f32> = Vec::new();
        let mut low_limits: Vec<f32> = Vec::new();
        let mut high_limits: Vec<f32> = Vec::new();
        let mut unitss: Vec<String> = Vec::new();

        for (tnum, mti) in &self.test_infos {
            test_nums.push(*tnum);
            test_types.push(mti.test_type.to_string());
            execution_counts.push(mti.execution_count);
            test_names.push(mti.test_name.clone());
            sequence_names.push(mti.sequence_name.clone());
            test_labels.push(mti.test_label.clone());
            test_times.push(mti.test_time);
            test_texts.push(mti.test_text.clone());
            llm_scals.push(mti.llm_scal as i32);
            hlm_scals.push(mti.hlm_scal as i32);
            res_scals.push(mti.res_scal as i32);
            lo_specs.push(mti.lo_spec);
            hi_specs.push(mti.hi_spec);
            low_limits.push(mti.low_limit);
            high_limits.push(mti.high_limit);
            unitss.push(mti.units.clone());
        }

        let mut columns: Vec<Column> = Vec::new();
        columns.push(Column::new("test_num".into(), test_nums));
        columns.push(Column::new("test_type".into(), test_types));
        columns.push(Column::new("execution_count".into(), execution_counts));
        columns.push(Column::new("test_name".into(), test_names));
        columns.push(Column::new("sequence_name".into(), sequence_names));
        columns.push(Column::new("test_label".into(), test_labels));
        columns.push(Column::new("test_time".into(), test_times));
        columns.push(Column::new("test_text".into(), test_texts));
        columns.push(Column::new("llm_scal".into(), llm_scals));
        columns.push(Column::new("hlm_scal".into(), hlm_scals));
        columns.push(Column::new("res_scal".into(), res_scals));
        columns.push(Column::new("lo_spec".into(), lo_specs));
        columns.push(Column::new("hi_spec".into(), hi_specs));
        columns.push(Column::new("low_limit".into(), low_limits));
        columns.push(Column::new("high_limit".into(), high_limits));
        columns.push(Column::new("units".into(), unitss));

        DataFrame::new(columns).unwrap()
    }
}
