use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use itertools::{enumerate, Itertools};

use crate::records::Records;
use crate::{
    records::records::{Record, FTR, PIR, PRR, PTR},
    test_information::{FullMergedTestInformation, FullTestInformation, TestType},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TestData {
    pub full_test_information: FullTestInformation,
    pub test_information: FullMergedTestInformation,
    pub index_lookup: HashMap<u32, usize>,
    pub data: Vec<Row>,
    pub temp_rows: HashMap<(u8, u8), Row>,
    n_para: usize,
    n_func: usize,
}

impl TestData {
    pub fn new(full_test_information: FullTestInformation) -> Self {
        let test_information = full_test_information.merge();

        // make lookup table: test_num -> index
        // where index is the index into the corresponding vector in Row
        let mut index_lookup = HashMap::new();
        let mut n_para: usize = 0;
        let mut n_func: usize = 0;
        for (tnum, mti) in test_information.test_infos.iter().sorted_by_key(|x| x.0) {
            if let TestType::P = mti.test_type {
                index_lookup.insert(*tnum, n_para);
                n_para += 1;
            }
            if let TestType::F = mti.test_type {
                index_lookup.insert(*tnum, n_func);
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
