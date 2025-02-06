use wasm_bindgen::prelude::*;

mod tailfrick;

#[wasm_bindgen]
pub fn run(program: &str) -> String {
    let program = tailfrick::Program::from(program);
    program.run()
}
