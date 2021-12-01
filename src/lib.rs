mod intcode_interpreter;

use wasm_bindgen::prelude::*;
use crate::intcode_interpreter::Program;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn run_intcode(input: &str) -> Program {
    let mut program = Program::parse(input);
    program.run()
}
