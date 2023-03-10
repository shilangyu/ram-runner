use std::{env::args, fs::read_to_string};

mod vm;
use vm::*;

fn main() {
    let mut args = args().skip(1);
    let file = args.next().unwrap();

    let initial_registers = args
        .map(|e| {
            let mut s = e.split('=');
            (s.next().unwrap().to_string(), s.next().unwrap().to_string())
        })
        .collect();

    let tokens = lex(&read_to_string(file).unwrap());

    let program = parse(tokens).expect("Parsing failed");

    let final_registers = run(program, initial_registers).expect("Runtime error");

    for (reg, val) in final_registers {
        if !val.is_empty() {
            println!("{reg}: {}", val.iter().collect::<String>());
        }
    }
}
