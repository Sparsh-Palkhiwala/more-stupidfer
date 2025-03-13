use std::collections::HashMap;
use std::env;

use itertools::Itertools;
use stupidf::data::TestData;
use stupidf::records::{RecordSummary, Records, records::Record};
use stupidf::test_information::{FullMergedTestInformation, FullTestInformation};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = args[1].clone();

    let records = Records::new(&fname)?;

    let mut summary = RecordSummary::new();
    let mut test_info = FullTestInformation::new();

    for record in records {
        summary.add(&record);
        if let Some(resolved) = record.resolve() {
            let header = &record.header;

            println!(
                "{}.{} (0x{:x} @ 0x{:x}): {:?}",
                header.rec_typ, header.rec_sub, header.rec_len, record.offset, record.rtype
            );
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
            println!("{resolved:#?}");
        }
    }
    println!("{summary:#?}");

    let mut merged_test_info = FullMergedTestInformation::new();
    for ti in test_info.test_infos.values() {
        merged_test_info.add_from_test_information(ti);
    }

    for mti in merged_test_info
        .test_infos
        .values()
        .sorted_by_key(|&x| (x.execution_count, x.test_num))
    {
        println!(
            "{:5} [{:?}]: {:5} ({})",
            mti.test_num, mti.test_type, mti.execution_count, mti.test_name
        );
    }

    let mut test_num_to_keys: HashMap<u32, Vec<(u8, u8)>> = HashMap::new();
    for key in test_info.test_infos.keys() {
        let (test_num, site_num, head_num) = *key;
        (*test_num_to_keys.entry(test_num).or_insert(Vec::new())).push((site_num, head_num));
    }

    for (key, value) in &mut test_num_to_keys.iter_mut().sorted() {
        value.sort();
        println!("{key}: {value:?}");
    }

    let mut test_data = TestData::new(test_info);
    let records = Records::new(&fname)?;

    for record in records {
        if let Some(resolved) = record.resolve() {
            if let Record::PIR(ref pir) = resolved {
                test_data.new_part(&pir);
            }
            if let Record::PTR(ref ptr) = resolved {
                test_data.add_data_ptr(&ptr);
            }
            if let Record::PRR(ref prr) = resolved {
                test_data.finish_part(&prr);
            }
        }
    }
    println!("{test_data:#?}");

    Ok(())
}
