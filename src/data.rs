use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};

use itertools::Itertools;
use polars::prelude::*;
use pyo3::IntoPyObject;

use crate::records::{Records, records::MIR};
use crate::{
    records::records::{FTR, PIR, PRR, PTR, Record},
    test_information::{FullMergedTestInformation, FullTestInformation, TestType},
};

/// `Row` describes the test results for an individually tested device
///
/// Defaults `x_coord` = `y_coord` = -5000 and `sbin` = `hbin` = 0. Parametric tests have a
/// default value of `NAN` and functional tests default to `false`.
#[derive(Debug, IntoPyObject)]
pub struct Row {
    part_id: String,
    x_coord: i16,
    y_coord: i16,
    head_num: u8,
    site_num: u8,
    sbin: u16,
    hbin: u16,
    results_parametric: Vec<f32>,
    results_functional: Vec<bool>,
}

impl Row {
    /// Create a new `Row` with pre-allocated space for the parametric and functional tests
    ///
    /// Each `Row` does not contain the test information metadata, so the number of functional and
    /// parametric tests must be specified manually. Creation is typically handled by `TestData`.
    ///
    /// Defaults `x_coord` = `y_coord` = -5000 and `sbin` = `hbin` = 0. Parametric tests have a
    /// default value of `NAN` and functional tests default to `false`.
    ///
    /// Space for the test results for every test is pre-allocated, but they are stored in a
    /// `Vec` for efficiency. The `test_number` -> index lookup is not contained in the `Row`, so
    /// a higher layer of abstraction (`TestData`) is required to actually add new data.
    pub fn new(pir: &PIR, num_tests_parametric: usize, num_tests_functional: usize) -> Self {
        Self {
            part_id: String::new(),
            x_coord: -5000,
            y_coord: -5000,
            head_num: pir.head_num,
            site_num: pir.site_num,
            sbin: 0,
            hbin: 0,
            results_parametric: vec![f32::NAN; num_tests_parametric as usize],
            results_functional: vec![false; num_tests_functional as usize],
        }
    }
}

/// `TestData` contains all of the test results and test information metadata
///
/// Both the merged (`test_information`) and unmerged (`full_test_information`) test metadata is
/// stored.
///
/// `index_lookup` maps the `test_num` -> index in the contained `Row`s. Since the test result
/// record (`PTR` or `FTR`) already specify whether it's a parametric or functional test, it's not
/// necessary to store this information. Therefore `index_lookup` is not one-to-one. Each
/// `test_num` will be either a parametric or a functional test.
///
/// A set of temporary `Row`s is held during iteration to track results. By the end of the STDF,
/// all temporary `Row`s should have been moved into `data`.
///
/// `TestData` implements the `Into<DataFrame>` trait and may be converted into a polars
/// `DataFrame`. The resulting `DataFrame` contains the test results, but not the test
/// information metadata. The test information metadata may be gathered by converting the
/// `test_information` to a `DataFrame`.
#[derive(Debug, IntoPyObject)]
pub struct TestData {
    /// The test information metadata indexed by (`test_num`, `site_num`, `head_num`)
    pub full_test_information: FullTestInformation,
    /// The test information metadata indexed by `test_num`
    pub test_information: FullMergedTestInformation,
    /// Mapping the `test_num` to `Row.results_parametric` or `Row.results_functional`
    pub index_lookup: HashMap<u32, usize>,
    /// The list of test results contained in `Row`s
    pub data: Vec<Row>,
    // The temporary rows indexed by (`test_num`, `site_num`, `head_num`)
    temp_rows: HashMap<(u8, u8), Row>,
    // The number of parametric tests
    n_para: usize,
    // The number of functional tests
    n_func: usize,
    // The mapping of index in `Row.results_parametric` to `test_num`
    reverse_lookup_para: HashMap<usize, u32>,
    // The mapping of index in `Row.results_functional` to `test_num`
    reverse_lookup_func: HashMap<usize, u32>,
}

impl TestData {
    /// Generates a `TestData` struct from the test information metadata
    pub fn new(full_test_information: FullTestInformation) -> Self {
        let test_information = full_test_information.merge();

        let mut index_lookup = HashMap::new();
        let mut reverse_lookup_para = HashMap::new();
        let mut reverse_lookup_func = HashMap::new();
        let mut n_para: usize = 0;
        let mut n_func: usize = 0;
        for (tnum, mti) in test_information.test_infos.iter().sorted_by_key(|x| x.0) {
            if let TestType::P = mti.test_type {
                index_lookup.insert(*tnum, n_para);
                reverse_lookup_para.insert(n_para, *tnum);
                n_para += 1;
            }
            if let TestType::F = mti.test_type {
                index_lookup.insert(*tnum, n_func);
                reverse_lookup_func.insert(n_func, *tnum);
                n_func += 1;
            }
        }

        let data = Vec::new();
        let temp_rows = HashMap::new();
        Self {
            full_test_information,
            test_information,
            index_lookup,
            data,
            temp_rows,
            n_para,
            n_func,
            reverse_lookup_para,
            reverse_lookup_func,
        }
    }

    /// Initializes a temporary new `Row` from a `PIR` indexed by
    /// (`test_num`, `site_num`, `head_num`)
    ///
    /// The previous temporary row must have been moved to `data` prior to this. Ingesting a `PRR`
    /// triggers moving the temporary row to `data`.
    pub fn new_part(&mut self, pir: &PIR) {
        let key = (pir.head_num, pir.site_num);
        if let Vacant(row) = self.temp_rows.entry(key) {
            row.insert(Row::new(&pir, self.n_para, self.n_func));
        } else {
            panic!("opening a specific head_num/site_num before closing the previous one!")
        }
    }

    /// Adds a parametric test result contained in the `PTR` to the appropriate temporary `Row`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    pub fn add_data_ptr(&mut self, ptr: &PTR) {
        let key = (ptr.head_num, ptr.site_num);
        let result = ptr.result;
        if let Occupied(mut row) = self.temp_rows.entry(key) {
            let results = &mut row.get_mut().results_parametric;
            let index = self
                .index_lookup
                .get(&ptr.test_num)
                .expect("found PTR with unknown test_num!");
            results[*index] = result;
        } else {
            panic!("trying to add data to a head_num/site_num that is not open!")
        }
    }

    /// Adds a functional test result contained in the `FTR` to the appropriate temporary `Row`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    pub fn add_data_ftr(&mut self, ftr: &FTR) {
        let key = (ftr.head_num, ftr.site_num);
        let result = ftr.get_passfail();
        if let Occupied(mut row) = self.temp_rows.entry(key) {
            let results = &mut row.get_mut().results_functional;
            let index = self
                .index_lookup
                .get(&ftr.test_num)
                .expect("found PTR with unknown test_num!");
            results[*index] = result;
        } else {
            panic!("trying to add data to a head_num/site_num that is not open!")
        }
    }

    /// Finalizes a set of test results for a given part specified by a `PRR`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    ///
    /// Much of the metadata in a `Row` is contained in the `PRR`, so this metadata is also added
    /// here.
    pub fn finish_part(&mut self, prr: &PRR) {
        let key = (prr.head_num, prr.site_num);
        if let Occupied(value) = self.temp_rows.entry(key) {
            let mut row = value.remove();
            row.part_id = prr.part_id.clone();
            row.x_coord = prr.x_coord;
            row.y_coord = prr.y_coord;
            row.sbin = prr.soft_bin;
            row.hbin = prr.hard_bin;
            self.data.push(row);
        } else {
            panic!("trying to close out a head_num/site_num that is not open!")
        }
    }

    /// Generate the `TestData` from an STDF file specified by `fname`
    ///
    /// Optionally allows for printing the record information with the `verbose` flag.
    ///
    /// Will traverse the STDF file twice: once to determine the test information metadata
    /// (required to pre-allocate the space for the tests in each `Row`), then again to actually
    /// capture the test results.
    ///
    /// # Error
    /// If for some reason the file specified by `fname` cannot be parsed, returns a
    /// `std::io::Error`
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let test_info = FullTestInformation::from_fname(fname, verbose)?;
        let mut test_data = Self::new(test_info);
        let records = Records::new(&fname)?;

        for record in records {
            if let Some(resolved) = record.resolve() {
                if let Record::PIR(ref pir) = resolved {
                    test_data.new_part(&pir);
                }
                if let Record::PTR(ref ptr) = resolved {
                    test_data.add_data_ptr(&ptr);
                }
                if let Record::FTR(ref ftr) = resolved {
                    test_data.add_data_ftr(&ftr);
                }
                if let Record::PRR(ref prr) = resolved {
                    test_data.finish_part(&prr);
                }
            }
        }
        Ok(test_data)
    }
}

/// Converts a `&TestData` into a `DataFrame` containing a tabular listing of all test results
impl Into<DataFrame> for &TestData {
    fn into(self) -> DataFrame {
        let mut part_ids: Vec<String> = Vec::new();
        let mut x_coords: Vec<i16> = Vec::new();
        let mut y_coords: Vec<i16> = Vec::new();
        let mut head_nums: Vec<u8> = Vec::new();
        let mut sbins: Vec<u16> = Vec::new();
        let mut hbins: Vec<u16> = Vec::new();
        let mut para_vecs: HashMap<u32, Vec<f32>> = HashMap::new();
        let mut func_vecs: HashMap<u32, Vec<bool>> = HashMap::new();
        let ncols_para = self.n_para;
        let ncols_func = self.n_func;
        for row in &self.data {
            part_ids.push(row.part_id.clone());
            x_coords.push(row.x_coord);
            y_coords.push(row.y_coord);
            head_nums.push(row.head_num);
            sbins.push(row.sbin);
            hbins.push(row.hbin);
            for i in 0..ncols_para {
                let test_num = self.reverse_lookup_para.get(&i).unwrap();
                para_vecs
                    .entry(*test_num)
                    .or_insert(Vec::new())
                    .push(row.results_parametric[i]);
            }
            for i in 0..ncols_func {
                let test_num = self.reverse_lookup_func.get(&i).unwrap();
                func_vecs
                    .entry(*test_num)
                    .or_insert(Vec::new())
                    .push(row.results_functional[i]);
            }
        }
        let mut columns: Vec<Column> = Vec::new();
        columns.push(Column::new("part_id".into(), part_ids));
        columns.push(Column::new("x_coords".into(), x_coords));
        columns.push(Column::new("y_coords".into(), y_coords));
        columns.push(Column::new("head_nums".into(), head_nums));
        columns.push(Column::new("sbins".into(), sbins));
        columns.push(Column::new("hbins".into(), hbins));
        for (test_num, vec) in para_vecs.iter().sorted_by_key(|x| x.0) {
            columns.push(Column::new(test_num.to_string().into(), vec));
        }
        for (test_num, vec) in func_vecs.iter().sorted_by_key(|x| x.0) {
            columns.push(Column::new(test_num.to_string().into(), vec));
        }
        DataFrame::new(columns).unwrap()
        //
    }
}

/// `STDF` contains the STDF file metadata (`mir`) and the test results data (`test_data`)
///
/// # Example
/// ```
/// let verbose = false;
/// if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
///     let df: DataFrame = (&stdf.test_data).into();
///     let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
///     println!("{df:#?}");
///     println!("{df_fmti}");
///     }
/// ```
#[derive(Debug, IntoPyObject)]
pub struct STDF {
    /// The STDF file metadata
    pub mir: MIR,
    /// The test results and test information metadata
    pub test_data: TestData,
}

impl STDF {
    /// Parses an STDF file from the file specified by `fname`
    ///
    /// # Example
    /// ```
    /// let verbose = false;
    /// if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
    ///     let df: DataFrame = (&stdf.test_data).into();
    ///     let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
    ///     println!("{df:#?}");
    ///     println!("{df_fmti}");
    ///     }
    ///
    /// ```
    /// # Error
    /// If for some reason the file cannot be parsed, returns an `std::io::Error`
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let mir = MIR::from_fname(&fname)?;
        let test_data = TestData::from_fname(&fname, verbose)?;
        Ok(Self { mir, test_data })
    }
}
