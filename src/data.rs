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
    pub fn new(pir: &PIR, num_tests_parametric: usize, num_tests_functional: usize) -> Self {
        Self {
            part_id: String::new(),
            x_coord: 0,
            y_coord: 0,
            head_num: pir.head_num,
            site_num: pir.site_num,
            sbin: 0,
            hbin: 0,
            results_parametric: vec![f32::NAN; num_tests_parametric as usize],
            results_functional: vec![false; num_tests_functional as usize],
        }
    }
}

#[derive(Debug, IntoPyObject)]
pub struct TestData {
    pub full_test_information: FullTestInformation,
    pub test_information: FullMergedTestInformation,
    pub index_lookup: HashMap<u32, usize>,
    pub data: Vec<Row>,
    temp_rows: HashMap<(u8, u8), Row>,
    n_para: usize,
    n_func: usize,
    reverse_lookup_para: HashMap<usize, u32>,
    reverse_lookup_func: HashMap<usize, u32>,
}

impl TestData {
    pub fn new(full_test_information: FullTestInformation) -> Self {
        let test_information = full_test_information.merge();

        // make lookup table: test_num -> index
        // where index is the index into the corresponding vector in Row
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

    pub fn new_part(&mut self, pir: &PIR) {
        let key = (pir.head_num, pir.site_num);
        if let Vacant(row) = self.temp_rows.entry(key) {
            row.insert(Row::new(&pir, self.n_para, self.n_func));
        } else {
            panic!("opening a specific head_num/site_num before closing the previous one!")
        }
    }

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

impl Into<DataFrame> for TestData {
    fn into(self) -> DataFrame {
        //
    }
}

#[derive(Debug, IntoPyObject)]
pub struct STDF {
    pub mir: MIR,
    pub test_data: TestData,
}

impl STDF {
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let mir = MIR::from_fname(&fname)?;
        let test_data = TestData::from_fname(&fname, verbose)?;
        Ok(Self { mir, test_data })
    }
}

pub fn make_vec(test_data: &TestData) -> Vec<f32> {
    test_data
        .data
        .iter()
        .map(|x| x.results_parametric[0])
        .collect()
}

pub fn make_series(test_data: &TestData) -> Series {
    test_data
        .data
        .iter()
        .map(|x| x.results_parametric[0])
        .collect()
}

#[derive(Debug)]
pub struct STDFDataFrame {
    pub df: DataFrame,
}

impl STDFDataFrame {
    pub fn new(test_data: &TestData) -> Self {
        let mut part_ids: Vec<String> = Vec::new();
        let mut x_coords: Vec<i16> = Vec::new();
        let mut y_coords: Vec<i16> = Vec::new();
        let mut head_nums: Vec<u8> = Vec::new();
        let mut sbins: Vec<u16> = Vec::new();
        let mut hbins: Vec<u16> = Vec::new();
        let mut para_vecs: HashMap<u32, Vec<f32>> = HashMap::new();
        let mut func_vecs: HashMap<u32, Vec<bool>> = HashMap::new();
        let ncols_para = test_data.n_para;
        let ncols_func = test_data.n_func;
        for row in &test_data.data {
            part_ids.push(row.part_id.clone());
            x_coords.push(row.x_coord);
            y_coords.push(row.y_coord);
            head_nums.push(row.head_num);
            sbins.push(row.sbin);
            hbins.push(row.hbin);
            for i in 0..ncols_para {
                let test_num = test_data.reverse_lookup_para.get(&i).unwrap();
                para_vecs
                    .entry(*test_num)
                    .or_insert(Vec::new())
                    .push(row.results_parametric[i]);
            }
            for i in 0..ncols_func {
                let test_num = test_data.reverse_lookup_func.get(&i).unwrap();
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
        let df = DataFrame::new(columns).unwrap();
        Self { df }
    }
}
