use std::collections::HashMap;

use crate::{
    data::{STDF, STDFDataFrame},
    records::records::MIR,
    test_information::{MergedTestInformation, TestInformation},
};
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

#[derive(IntoPyObject)]
struct PySTDF {
    mir: MIR,
    df: PyDataFrame,
    test_information: PyDataFrame,
    // #TODO add the FullTestInformation hashmap
    full_test_information: HashMap<(u32, u8, u8), TestInformation>,
}

impl PySTDF {
    fn from_fname(fname: &str) -> std::io::Result<Self> {
        let stdf = STDF::from_fname(&fname, false)?;
        let mir = stdf.mir;
        let df = PyDataFrame(STDFDataFrame::new(&stdf.test_data).df);
        let test_information = PyDataFrame((&stdf.test_data.test_information).into());
        let full_test_information = stdf.test_data.full_test_information.test_infos;
        Ok(Self {
            mir,
            df,
            test_information,
            full_test_information,
        })
    }
}

#[pyfunction]
fn parse_stdf(fname: &str) -> PyResult<PySTDF> {
    let pystdf = PySTDF::from_fname(&fname)?;
    Ok(pystdf)
}

#[pyfunction]
fn get_mir(fname: &str) -> PyResult<MIR> {
    let mir = MIR::from_fname(&fname)?;
    Ok(mir)
}

#[pymodule]
fn stupidf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_mir, m)?)?;
    m.add_function(wrap_pyfunction!(parse_stdf, m)?)?;
    Ok(())
}
