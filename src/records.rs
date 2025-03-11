use crate::records::records::*;
use std::{
    collections::HashMap,
    f32,
    fs::File,
    io::{self, BufReader, Read},
};

use crate::record_types::RecordType;
pub mod records;

#[derive(Debug)]
pub struct Header {
    pub rec_len: u16,
    pub rec_typ: u8,
    pub rec_sub: u8,
}

impl Header {
    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        Self {
            rec_len: u16::from_le_bytes(bytes[..2].try_into().unwrap()),
            rec_typ: bytes[2],
            rec_sub: bytes[3],
        }
    }

    pub fn from_file(reader: &mut impl Read) -> Result<Self, io::Error> {
        let mut buf: [u8; 4] = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(Header::from_bytes(&buf))
    }
}

#[derive(Debug)]
pub struct RawRecord {
    pub header: Header,
    pub offset: usize,
    pub contents: Vec<u8>,
    pub rtype: RecordType,
}

impl RawRecord {
    pub fn from_header(
        header: Header,
        reader: &mut impl Read,
        offset: usize,
    ) -> Result<Self, io::Error> {
        let rtype = RecordType::new(header.rec_typ, header.rec_sub);
        let mut contents = vec![0u8; header.rec_len as usize];
        reader.read_exact(&mut contents)?;
        Ok(Self {
            header,
            offset,
            contents,
            rtype,
        })
    }

    // could Record enum be swapped for trait object?
    pub fn resolve(&self) -> Option<Record> {
        match self.rtype {
            RecordType::MIR => Some(Record::MIR(MIR::from_raw_record(&self))),
            RecordType::SDR => Some(Record::SDR(SDR::from_raw_record(&self))),
            RecordType::TSR => Some(Record::TSR(TSR::from_raw_record(&self))),
            RecordType::SBR => Some(Record::SBR(SBR::from_raw_record(&self))),
            RecordType::HBR => Some(Record::HBR(HBR::from_raw_record(&self))),
            RecordType::PCR => Some(Record::PCR(PCR::from_raw_record(&self))),
            RecordType::MRR => Some(Record::MRR(MRR::from_raw_record(&self))),
            RecordType::PIR => Some(Record::PIR(PIR::from_raw_record(&self))),
            RecordType::PRR => Some(Record::PRR(PRR::from_raw_record(&self))),
            RecordType::WIR => Some(Record::WIR(WIR::from_raw_record(&self))),
            RecordType::WRR => Some(Record::WRR(WRR::from_raw_record(&self))),
            RecordType::PTR => Some(Record::PTR(PTR::from_raw_record(&self))),
            _ => None,
        }
    }
}

impl std::fmt::Display for RawRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = String::from_utf8_lossy(&self.contents);
        write!(f, "Record({0:?}, contents: {s})", &self.header)?;
        Ok(())
    }
}

pub struct Records {
    reader: BufReader<File>,
    offset: usize,
}

impl Records {
    pub fn new(fname: &str) -> std::io::Result<Self> {
        let f = File::open(&fname)?;
        let reader = BufReader::new(f);
        Ok(Self { reader, offset: 0 })
    }
}

impl Iterator for Records {
    type Item = RawRecord;

    fn next(&mut self) -> Option<Self::Item> {
        match Header::from_file(&mut self.reader) {
            Ok(header) => {
                self.offset += header.rec_len as usize;
                RawRecord::from_header(header, &mut self.reader, self.offset).ok()
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct RecordSummary {
    counts: HashMap<RecordType, i32>,
}

impl RecordSummary {
    pub fn new() -> Self {
        let counts = HashMap::new();
        Self { counts }
    }

    pub fn add(&mut self, raw_record: &RawRecord) {
        let count = self.counts.entry(raw_record.rtype).or_insert(0);
        *count += 1;
    }
}

#[derive(Debug)]
pub struct TestInformation {
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

impl TestInformation {
    pub fn new(tsr: &TSR) -> Self {
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

        Self {
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

    pub fn add_from_ptr(&mut self, ptr: &PTR) {
        self.test_text = ptr.test_txt.clone();
        self.low_limit = ptr.lo_limit;
        self.high_limit = ptr.hi_limit;
        self.units = ptr.units.clone();
    }
}

#[derive(Debug)]
pub enum TestType {
    P,
    F,
    M,
    S,
    Unknown,
}

#[derive(Debug)]
pub struct PartData {
    data: HashMap<String, Vec<f32>>,
}
