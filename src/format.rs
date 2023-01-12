use crate::vm::Line;

pub fn format(lines: Vec<Line>) -> String {
    let mut indent = None;
    let mut formatted = String::new();

    for l in lines {
        if let Some(label) = &l.label {
            formatted.push_str(&format!(
                "{}{label}: ",
                if indent.is_some() { "\n" } else { "" }
            ));
            indent = Some(label.len() + 2);
        }

        formatted.push_str(&format!(
            "{}{}\n",
            " ".repeat(if l.label.is_some() {
                0
            } else {
                indent.unwrap_or(0)
            }),
            match l.instruction {
                crate::vm::Instruction::Clr(reg) => format!("clr {reg}"),
                crate::vm::Instruction::Del(reg) => format!("del {reg}"),
                crate::vm::Instruction::Add(reg, c) => format!("add{c} {reg}"),
                crate::vm::Instruction::Copy(dest, source) => format!("{dest} <- {source}"),
                crate::vm::Instruction::Jmp(reg) => format!("jmp {reg}"),
                crate::vm::Instruction::CondJmp(reg, c, label) => format!("{reg} jmp{c} {label}"),
                crate::vm::Instruction::Continue => "continue".to_string(),
            }
        ));
    }

    formatted
}
