use crate::records::records::PTR;
use crate::records::records::TSR;
use std::collections::HashMap;

#[derive(Debug)]
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
    pub low_limit: f32,
    pub high_limit: f32,
    pub units: String,
    pub complete: Complete,
}

#[derive(Debug)]
pub enum Complete {
    //None,
    PTR,
    TSR,
    Complete,
}

impl TestInformation {
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
        let test_text = ptr.test_txt.clone();
        let low_limit = ptr.lo_limit;
        let high_limit = ptr.hi_limit;
        let units = ptr.units.clone();
        let complete = Complete::PTR;

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
            low_limit,
            high_limit,
            units,
            complete,
        }
    }

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
        let low_limit = f32::NAN;
        let high_limit = f32::NAN;
        let units = String::new();
        let complete = Complete::TSR;

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
            low_limit,
            high_limit,
            units,
            complete,
        }
    }

    pub fn add_from_ptr(&mut self, ptr: &PTR) {
        if (self.head_num != ptr.head_num)
            || (self.site_num != ptr.site_num)
            || (self.test_num != ptr.test_num)
        {
            panic!("head_num/site_num/test_num from PTR does not match!");
        }
        if let Complete::TSR = self.complete {
            self.test_text = ptr.test_txt.clone();
            self.low_limit = ptr.lo_limit;
            self.high_limit = ptr.hi_limit;
            self.units = ptr.units.clone();
            self.complete = Complete::Complete;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum TestType {
    P,
    F,
    M,
    S,
    Unknown,
}

#[derive(Debug)]
pub struct FullTestInformation {
    pub test_infos: HashMap<(u32, u8, u8), TestInformation>,
}

impl FullTestInformation {
    pub fn new() -> Self {
        let test_infos = HashMap::new();
        Self { test_infos }
    }

    pub fn add_from_ptr(&mut self, ptr: &PTR) {
        let key = (ptr.test_num, ptr.site_num, ptr.head_num);
        self.test_infos
            .entry(key)
            .and_modify(|e| e.add_from_ptr(ptr))
            .or_insert(TestInformation::new_from_ptr(ptr));
    }

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

    pub fn merge(&self) -> FullMergedTestInformation {
        let mut merged_test_info = FullMergedTestInformation::new();
        for ti in self.test_infos.values() {
            merged_test_info.add_from_test_information(ti);
        }
        merged_test_info
    }
}

impl IntoIterator for FullTestInformation {
    type Item = ((u32, u8, u8), TestInformation);
    type IntoIter = <HashMap<(u32, u8, u8), TestInformation> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.test_infos.into_iter()
    }
}

//impl<'a> IntoIterator for &'a FullTestInformation {
//    type Item = ((u32, u8, u8), &'a TestInformation);
//    type IntoIter = <HashMap<(u32, u8, u8), &'a TestInformation> as IntoIterator>::IntoIter;
//    fn into_iter(self) -> Self::IntoIter {
//        self.test_infos.iter()
//    }
//}

#[derive(Debug)]
pub struct MergedTestInformation {
    pub test_num: u32,
    pub test_type: TestType,
    pub execution_count: u32,
    pub test_name: String,
    pub sequence_name: String,
    pub test_label: String,
    pub test_time: f32,
    pub test_text: String,
    pub low_limit: f32,
    pub high_limit: f32,
    pub units: String,
}
impl MergedTestInformation {
    pub fn new_from_test_information(test_information: &TestInformation) -> Self {
        let test_num = test_information.test_num;
        let test_type = test_information.test_type.clone();
        let execution_count = test_information.execution_count;
        let test_name = test_information.test_name.clone();
        let sequence_name = test_information.sequence_name.clone();
        let test_label = test_information.test_label.clone();
        let test_time = test_information.test_time;
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
            low_limit,
            high_limit,
            units,
        }
    }
    pub fn add(&mut self, test_information: &TestInformation) {
        if self.test_num != test_information.test_num {
            panic!("TestInformation.test_num does not match that of MergedTestInformation!")
        }
        self.execution_count += test_information.execution_count;
    }
}

#[derive(Debug)]
pub struct FullMergedTestInformation {
    pub test_infos: HashMap<u32, MergedTestInformation>,
}
impl FullMergedTestInformation {
    pub fn new() -> Self {
        let test_infos = HashMap::new();
        Self { test_infos }
    }

    pub fn add_from_test_information(&mut self, test_information: &TestInformation) {
        let key = test_information.test_num;
        self.test_infos
            .entry(key)
            .and_modify(|e| e.add(test_information))
            .or_insert(MergedTestInformation::new_from_test_information(
                test_information,
            ));
    }

    pub fn get_num(&self, test_type: TestType) -> usize {
        self.test_infos
            .values()
            .filter(|&mti| mti.test_type == test_type)
            .collect::<Vec<_>>()
            .len()
    }
}
