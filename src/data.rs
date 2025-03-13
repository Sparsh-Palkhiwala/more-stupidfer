use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};

use itertools::{Itertools, enumerate};

use crate::{
    records::records::{PIR, PRR, PTR},
    test_information::{FullMergedTestInformation, FullTestInformation},
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
    results: Vec<f32>,
}

impl Row {
    pub fn new(pir: &PIR, num_tests: usize) -> Self {
        Self {
            part_id: String::new(),
            x_coord: 0,
            y_coord: 0,
            head_num: pir.head_num,
            site_num: pir.site_num,
            sbin: 0,
            hbin: 0,
            results: vec![f32::NAN; num_tests as usize],
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
}

impl TestData {
    pub fn new(full_test_information: FullTestInformation) -> Self {
        let test_information = full_test_information.merge();
        let mut index_lookup: HashMap<u32, usize> = HashMap::new();
        for (i, key) in enumerate(test_information.test_infos.keys().sorted()) {
            index_lookup.insert(*key, i);
        }
        let data = Vec::new();
        let temp_rows = HashMap::new();
        Self {
            full_test_information,
            test_information,
            index_lookup,
            data,
            temp_rows,
        }
    }
    pub fn new_part(&mut self, pir: &PIR) {
        let num_tests = self.test_information.test_infos.len();
        let key = (pir.head_num, pir.site_num);
        if let Vacant(value) = self.temp_rows.entry(key) {
            value.insert(Row::new(&pir, num_tests));
        } else {
            panic!("opening a specific head_num/site_num before closing the previous one!")
        }
    }

    pub fn add_data(&mut self, ptr: &PTR) {
        let key = (ptr.head_num, ptr.site_num);
        let result = ptr.result;
        if let Occupied(mut value) = self.temp_rows.entry(key) {
            let results = &mut value.get_mut().results;
            let index = self
                .index_lookup
                .get(&ptr.test_num)
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
}
