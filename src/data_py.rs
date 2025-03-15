use crate::{
    data::{STDFDataFrame, TestData, make_series, make_vec},
    records::records::MIR,
};
use pyo3::prelude::*;
use pyo3_polars::{PyDataFrame, PySeries};

use crate::data::STDF;

#[pyfunction]
fn get_mir(fname: &str) -> PyResult<MIR> {
    let mir = MIR::from_fname(&fname)?;
    Ok(mir)
}

#[pyfunction]
fn get_stdf(fname: &str) -> PyResult<STDF> {
    let stdf = STDF::from_fname(&fname, false)?;
    Ok(stdf)
}

#[pyfunction]
fn get_df() -> PyResult<PyDataFrame> {
    let df = STDFDataFrame::test().df;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn get_vec(fname: &str) -> PyResult<Vec<f32>> {
    let test_data = TestData::from_fname(&fname, false)?;
    let series = make_vec(&test_data);
    Ok(series)
}

#[pyfunction]
fn get_series(fname: &str) -> PyResult<PySeries> {
    let test_data = TestData::from_fname(&fname, false)?;
    let series = make_series(&test_data);
    Ok(PySeries(series))
}

/// A Python module implemented in Rust.
#[pymodule]
fn stupidf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_mir, m)?)?;
    m.add_function(wrap_pyfunction!(get_stdf, m)?)?;
    m.add_function(wrap_pyfunction!(get_df, m)?)?;
    m.add_function(wrap_pyfunction!(get_vec, m)?)?;
    m.add_function(wrap_pyfunction!(get_series, m)?)?;
    Ok(())
}
