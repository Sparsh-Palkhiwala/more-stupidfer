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
    data::{MasterInformation, Row, STDF, TestData, WaferInformation},
    records::records::*,
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
    wafers: Vec<WaferInformation>,
    /// The site information
    site_information: SDR,
    /// The soft-bin information
    soft_bins: PyDataFrame,
    /// The hard-bin information
    hard_bins: PyDataFrame,
    /// The pin mapping information
    pins: PyDataFrame,
    /// The test number -> [pin_id] mapping
    pin_mapping: HashMap<u32, Vec<u16>>,
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
        let site_information = stdf.site_information.clone();
        let soft_bins = PyDataFrame(stdf.soft_bins_to_df());
        let hard_bins = PyDataFrame(stdf.hard_bins_to_df());
        let pins = PyDataFrame(stdf.pin_mapping_to_df());
        let pin_mapping = stdf.test_data.mpr_index_lookup.clone();
        let test_data = &stdf.test_data;
        let test_info = &test_data.test_information;
        let df = PyDataFrame(test_data.into());
        let test_information = PyDataFrame(test_info.into());
        let full_test_information = stdf.test_data.full_test_information.test_infos;
        Ok(Self {
            metadata,
            wafers,
            site_information,
            soft_bins,
            hard_bins,
            pins,
            pin_mapping,
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

/// get_rows(fname: str)
/// --
///
/// Parse an STDF file specified by `fname` and return a list of rows
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a list of dicts, where each dict represent a single row (i.e. part).
/// Useful if you need only the row-formatted data. The list is fully realized,
/// i.e. a proper list, not a generator.
///
/// # Example
/// ```
///    import stupidf as sf
///    rows = sf.get_rows("my_stdf.stdf")
///    rows[0]
/// ````
#[pyfunction]
fn get_rows(fname: &str) -> PyResult<Vec<Row>> {
    let test_data = TestData::from_fname(fname, false)?;
    Ok(test_data.data)
}

/// get_raw_stdf(fname: str)
/// --
///
/// Parse an STDF file specified by `fname` into a dict structure
///
/// `fname` must be a `str` and may not be a `Path`-like object.
///
/// Returns a nested `dict` representing the raw rust STDF object. Useful if you
/// do not need the DataFrame representation and prefer a row-formatted representation.
/// The entire `dict` is fully realized, i.e. there are no generators.
///    `master_information`: `dict` describing the Master Infomation Record and Master
///        Results Record (file metadata)
///    `wafer_information`: `dict` describing the Wafer Information Records and Wafer
///        Results Records (wafer metadata)
///    `site_information`: `dict` describing site information
///    `soft_bins`: `dict` of {sbin: SBR}
///    `hard_bins`: `dict` of {hbin: HBR}
///    `pins`: `dict` of {pin_index: PMR}
///    `test_data`: a `dict` describing all of the test results
///
/// # Example
/// ```
///    import stupidf as sf
///    raw_stdf = sf.get_raw_stdf("my_stdf.stdf")
///    raw_stdf['master_information']
/// ````
#[pyfunction]
fn get_raw_stdf(fname: &str) -> PyResult<STDF> {
    let stdf = STDF::from_fname(fname, false)?;
    Ok(stdf)
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
    m.add_function(wrap_pyfunction!(get_rows, m)?)?;
    m.add_function(wrap_pyfunction!(get_raw_stdf, m)?)?;
    Ok(())
}
