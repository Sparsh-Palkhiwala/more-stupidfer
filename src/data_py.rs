//! `stupidf_py` contains the Python bindings and code for passing the data to Python
//!
//! The relevant function is `stupidf.parse_stdf`
//!
//! # Example
//! ```
//! import stupidf as sf
//! stdf = sf.parse_stdf("my_stdf.stdf")
//! stdf['df']
//! ```
use std::collections::HashMap;

use crate::{
    data::{MasterInformation, STDF, WaferInformation},
    records::records::MIR,
    test_information::TestInformation,
};
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

/// A wrapper for the STDF suitable for throwing across the barrier to Python land
#[derive(IntoPyObject)]
struct PySTDF {
    /// MIR and MRR information
    metadata: MasterInformation,
    /// WIR and WRR information
    wafer_information: Vec<WaferInformation>,
    /// The `DataFrame` containing the test results (corresponds to `TestData`)
    df: PyDataFrame,
    /// The `DataFrame` containing the test information metadata (corresponds to
    /// `FullMergedTestInformation`)
    test_information: PyDataFrame,
    /// A dict containing the full test information metadata indexed by
    /// (`test_num`, `site_num`, `head_num`)
    full_test_information: HashMap<(u32, u8, u8), TestInformation>,
}

impl PySTDF {
    /// Generates the PySTDF from a file specified by `fname`
    ///
    /// Analagous to `STDF::from_fname`
    fn from_fname(fname: &str) -> std::io::Result<Self> {
        let stdf = STDF::from_fname(&fname, false)?;
        let metadata = stdf.master_information.clone();
        let wafers = stdf.wafer_information.clone();
        let test_data = &stdf.test_data;
        let test_info = &test_data.test_information;
        let df = PyDataFrame(test_data.into());
        let test_information = PyDataFrame(test_info.into());
        let full_test_information = stdf.test_data.full_test_information.test_infos;
        Ok(Self {
            metadata,
            wafer_information: wafers,
            df,
            test_information,
            full_test_information,
        })
    }
}

/// parse_stdf(fname: str)
/// --
///
/// Parse an STDF file specified by `fname`
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a dict with keys and values:
///    `mir`: `dict` describing the Master Infomation Record (file metadata)
///    `df`: `DataFrame` containing the test results
///    `test_information`: `DataFrame` containing the merged test information metadata
///    `full_test_information`: `dict` containing the full test information metadata
///
/// # Example
/// ```
///    import stupidf as sf
///    stdf = sf.parse_stdf("my_stdf.stdf")
///    stdf['df']
/// ````
#[pyfunction]
fn parse_stdf(fname: &str) -> PyResult<PySTDF> {
    let pystdf = PySTDF::from_fname(&fname)?;
    Ok(pystdf)
}

/// get_mir(fname: str)
/// --
///
/// Get the MIR from the file specified by `fname`
///
/// `fname` must be a `str` and may not be a `Path`-like object.
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
