use crate::records::records::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
};

use crate::record_types::RecordType;
pub mod records;

/// A header for an STDF record
///
/// STDF files are singly linked lists, so the header describes the record type and where to find
/// the next record
///
/// The `rec_typ` and `rec_sub` uniquely determine the record type
///
/// `Headers` always contain exactly 4 bytes
///
/// The next record will always be (4 + `rec_len`) bytes after the start of the current record.
#[derive(Debug)]
pub struct Header {
    /// The number of bytes to the next record, from the end of the `Header`
    pub rec_len: u16,
    pub rec_typ: u8,
    pub rec_sub: u8,
}

impl Header {
    /// Generate a `Header` from an array of bytes
    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        Self {
            rec_len: u16::from_le_bytes(bytes[..2].try_into().unwrap()),
            rec_typ: bytes[2],
            rec_sub: bytes[3],
        }
    }

    /// Get the next `Header` from a `reader` (e.g. a file handle)
    pub fn from_file(reader: &mut impl Read) -> Result<Self, io::Error> {
        let mut buf: [u8; 4] = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(Header::from_bytes(&buf))
    }
}

/// The raw information in an STDF record
///
/// The owned `header: Header` determines the record type and specifies how many bytes are
/// contained in the record.
///
/// Contains the raw contents of the header as a `Vec<u8>` in `contents`, but does not yet parse
/// them out. See the `resolve` method for parsing a `RawRecord` into a concrete record type. The
/// record type is determined and stored though. Resolving a record is comparably expensive, so it
/// is done only on demand.
///
/// Also contains the location of the `RawRecord` in the reader (e.g. file)
#[derive(Debug)]
pub struct RawRecord {
    /// The owned record header
    pub header: Header,
    /// The location of the `RawRecord` in the file
    pub offset: usize,
    /// The raw unparsed contents of the `RawRecord`
    pub contents: Vec<u8>,
    /// The type of record contained in the `RawRecord`
    pub rtype: RecordType,
}

impl RawRecord {
    /// Given a record header, get the record contents and determine the record type
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

    /// Resolve a `RawRecord` into a concrete record type
    ///
    /// The record type is already contained in the `RawRecord`, so can immediately resolve to the
    /// conrete record without hitting the file again.
    ///
    /// Resolving is the most expensive part of the process, so it is done only on-demand.
    pub fn resolve(&self) -> Option<Record> {
        match self.rtype {
            RecordType::MIR => Some(Record::MIR(self.into())),
            RecordType::SDR => Some(Record::SDR(self.into())),
            RecordType::TSR => Some(Record::TSR(self.into())),
            RecordType::SBR => Some(Record::SBR(self.into())),
            RecordType::HBR => Some(Record::HBR(self.into())),
            RecordType::PCR => Some(Record::PCR(self.into())),
            RecordType::MRR => Some(Record::MRR(self.into())),
            RecordType::PIR => Some(Record::PIR(self.into())),
            RecordType::PRR => Some(Record::PRR(self.into())),
            RecordType::WIR => Some(Record::WIR(self.into())),
            RecordType::WRR => Some(Record::WRR(self.into())),
            RecordType::PTR => Some(Record::PTR(self.into())),
            RecordType::FTR => Some(Record::FTR(self.into())),
            RecordType::MPR => Some(Record::MPR(self.into())),
            RecordType::PMR => Some(Record::PMR(self.into())),
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

/// A helper struct for iterating through a buffered file reading and tracking the location
///
/// Iterating over `Records` yields `RawRecords` in the file
pub struct Records {
    reader: BufReader<File>,
    offset: usize,
}

impl Records {
    /// Create a new `Records` iterable from a filename `fname`
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

/// A summary of the number of records of each type.
///
/// Iterating over `RecordSummary` yields (`RecordType`, `num_records`)
#[derive(Debug)]
pub struct RecordSummary {
    counts: HashMap<RecordType, i32>,
}

impl RecordSummary {
    pub fn new() -> Self {
        let counts = HashMap::new();
        Self { counts }
    }

    /// Add a count corresponding to the `RecordType` in `RawRecord`
    pub fn add(&mut self, raw_record: &RawRecord) {
        let count = self.counts.entry(raw_record.rtype).or_insert(0);
        *count += 1;
    }
}

impl IntoIterator for RecordSummary {
    type Item = (RecordType, i32);
    type IntoIter = <HashMap<RecordType, i32> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}
