use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod vm;
use vm::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Registers(HashMap<String, String>);

#[wasm_bindgen]
pub fn run_program(program: &str, initial_registers: JsValue) -> String {
    let initial_registers: Registers = serde_wasm_bindgen::from_value(initial_registers).unwrap();

    let tokens = lex(program);

    let program = parse(tokens);

    let final_registers = run(program, initial_registers.0);

    let mut out = String::new();
    for (reg, val) in final_registers {
        out.push_str(&format!("{reg}: {}", val.iter().collect::<String>()));
    }
    out
}
