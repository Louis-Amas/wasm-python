use pyo3::{append_to_inittab, PyResult};

use wasm_python::call_function;
use wasm_python::py_module::make_person_module;

pub fn main() {
    append_to_inittab!(make_person_module);

    let function_code = include_str!("py-func.py");
    let result = call_function(
        "my_func",
        function_code,
        (
            ("John", 21, ["funny", "student"]),
            ("Jane", 22, ["thoughtful", "student"]),
            ("George", 75, ["wise", "retired"]),
        ),
    );

    println!("{result:?}");
}
