use pyo3::append_to_inittab;

use wasm_python::bindings::Guest;
use wasm_python::py_module::make_person_module;
use wasm_python::{bindings::export, call_function, Plugin};

struct TestStrategy;

impl Plugin for TestStrategy {
    fn execute(code: String) -> Result<(), String> {
        append_to_inittab!(make_person_module);

        let result = call_function(
            "my_func",
            &code,
            (
                ("John", 21, ["funny", "student"]),
                ("Jane", 22, ["thoughtful", "student"]),
                ("George", 75, ["wise", "retired"]),
            ),
        );

        println!("{result:?}");

        Ok(())
    }
}

impl Guest for TestStrategy {
    fn exec(code: String) -> Result<Vec<u8>, String> {
        Self::execute(code).unwrap();
        Ok(Default::default())
    }
}

export!(TestStrategy with_types_in wasm_python::bindings);
