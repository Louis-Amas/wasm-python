use std::ffi::CStr;

use pyo3::call::PyCallArgs;
use pyo3::ffi::c_str;
use pyo3::types::PyModule;
use pyo3::PyResult;
use pyo3::{append_to_inittab, prelude::*};

pub mod bindings {

    use wit_bindgen::generate;
    generate!({path: "strategy.wit", pub_export_macro: true, export_macro_name: "export", });
}
pub mod py_module;
pub mod py_rust_decimal;

use py_module::make_person_module;
use py_rust_decimal::make_decimal_module;

pub fn call_function<T>(function_name: &CStr, function_code: &CStr, args: T) -> PyResult<()>
where
    T: for<'py> PyCallArgs<'py>,
{
    append_to_inittab!(make_person_module);
    append_to_inittab!(make_decimal_module);
    Python::initialize();

    Python::attach(|py| {
        // from_code_bound is the 0.26 Bound API variant; &CStr works here
        let module =
            PyModule::from_code(py, function_code, c_str!("inline.py"), c_str!("inline_mod"))?;

        // getattr expects a PyString — convert &CStr -> &str (UTF-8). lossy is fine for attr names.
        let fun = module.getattr(function_name.to_string_lossy().as_ref())?;

        fun.call1(args)?;
        Ok(())
    })
}

pub trait Plugin {
    fn execute(code: String) -> Result<(), String>;
}
