pub mod data;
pub mod record_types;
pub mod records;
pub mod test_information;
mod util;

use data::STDF;
use pyo3::prelude::*;
use records::records::MIR;
use test_information::TestType;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn get_mir(fname: &str) -> PyResult<MIR> {
    let mir = MIR::from_fname(&fname)?;
    Ok(mir)
}

#[pyfunction]
fn get_TestType(test_type: &str) -> PyResult<TestType> {
    let tt = match test_type {
        "P" => TestType::P,
        "F" => TestType::F,
        "M" => TestType::M,
        "S" => TestType::S,
        _ => TestType::Unknown,
    };
    Ok(tt)
}

#[pyfunction]
fn get_stdf(fname: &str) -> PyResult<STDF> {
    let stdf = STDF::from_fname(&fname, false)?;
    Ok(stdf)
}

/// A Python module implemented in Rust.
#[pymodule]
fn stupidf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_mir, m)?)?;
    m.add_function(wrap_pyfunction!(get_TestType, m)?)?;
    m.add_function(wrap_pyfunction!(get_stdf, m)?)?;
    Ok(())
}
