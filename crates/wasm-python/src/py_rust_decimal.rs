use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[pyclass(name = "Decimal")]
#[derive(Clone)]
pub struct PyDecimal {
    inner: Decimal,
}

#[pymethods]
impl PyDecimal {
    #[new]
    fn new(s: &str) -> PyResult<Self> {
        match s.parse::<Decimal>() {
            Ok(d) => Ok(Self { inner: d }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Decimal(\"{}\")", self.inner)
    }

    /// Add two decimals
    fn add(&self, other: &PyDecimal) -> PyDecimal {
        PyDecimal {
            inner: self.inner + other.inner,
        }
    }

    /// Subtract two decimals
    fn sub(&self, other: &PyDecimal) -> PyDecimal {
        PyDecimal {
            inner: self.inner - other.inner,
        }
    }

    /// Multiply two decimals
    fn mul(&self, other: &PyDecimal) -> PyDecimal {
        PyDecimal {
            inner: self.inner * other.inner,
        }
    }

    /// Divide two decimals
    fn div(&self, other: &PyDecimal) -> PyResult<PyDecimal> {
        if other.inner.is_zero() {
            return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                "Division by zero",
            ));
        }
        Ok(PyDecimal {
            inner: self.inner / other.inner,
        })
    }

    /// Compare equality
    fn eq(&self, other: &PyDecimal) -> bool {
        self.inner == other.inner
    }

    /// Return as float (lossy)
    fn to_f64(&self) -> Option<f64> {
        f64::from_f64(self.inner.to_string().parse().ok()?)
    }
}

#[pymodule(gil_used = false)]
#[pyo3(name = "decimal_rs")]
pub fn make_decimal_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDecimal>()?;
    Ok(())
}
