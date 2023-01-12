use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod format;
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
pub fn run_program(program: &str, initial_registers: JsValue) -> Result<String, String> {
    let initial_registers: Registers = serde_wasm_bindgen::from_value(initial_registers).unwrap();

    let tokens = lex(program);

    let program = parse(tokens).map_err(|err| format!("Parsing error: {err}"))?;

    let final_registers =
        run(program, initial_registers.0).map_err(|err| format!("Runtime error: {err}"))?;

    let mut final_registers = final_registers.into_iter().collect::<Vec<_>>();
    final_registers.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = String::new();
    for (reg, val) in final_registers {
        if !val.is_empty() {
            out.push_str(&format!("{reg}: {}\n", val.iter().collect::<String>()));
        }
    }
    Ok(out)
}

#[wasm_bindgen]
pub fn format(program: &str) -> Result<String, String> {
    let tokens = lex(program);
    let program = parse(tokens).map_err(|err| format!("Parsing error: {err}"))?;

    Ok(format::format(program))
}
