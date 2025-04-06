use pyo3::prelude::*;

/// 加
#[pyfunction]
fn add(a: usize, b: usize) -> PyResult<i64> {
    let sum = a + b;

    Ok(sum as i64)
}

/// 减
#[pyfunction]
fn sub(a: usize, b: usize) -> PyResult<i64> {
    let sub = a - b;

    Ok(sub as i64)
}

/// 乘
#[pyfunction]
fn mul(a: usize, b: usize) -> PyResult<i64> {
    let mul = a * b;

    Ok(mul as i64)
}

/// 除
#[pyfunction]
fn div(a: usize, b: usize) -> PyResult<f64> {
    if b == 0 {
        return Err(pyo3::exceptions::PyValueError::new_err("b must not be 0"));
    }
    let div = a as f64 / b as f64;

    Ok(div as f64)
}

/// 计算器模块的实现
#[pymodule]
fn calculator(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1")?;

    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(sub, m)?)?;
    m.add_function(wrap_pyfunction!(mul, m)?)?;
    m.add_function(wrap_pyfunction!(div, m)?)?;
    Ok(())
}
