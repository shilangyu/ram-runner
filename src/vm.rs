use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum Instruction {
    Clr(String),
    Del(String),
    Add(String, char),
    Copy(String, String),
    Jmp(String),
    CondJmp(String, char, String),
    Continue,
}

#[derive(Debug)]
pub struct Line {
    label: Option<String>,
    instruction: Instruction,
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Label(String),
    Clr,
    Del,
    Add(char),
    Copy,
    Jmp,
    CondJmp(char),
    Continue,
}

pub fn lex(content: String) -> Vec<Token> {
    content
        .split_whitespace()
        .map(|e| match e {
            "clr" => Token::Clr,
            "del" => Token::Del,
            "<-" => Token::Copy,
            "jmp" => Token::Jmp,
            "continue" => Token::Continue,
            e => {
                if e.starts_with("jmp") && e.len() == 4 {
                    Token::CondJmp(e.chars().last().unwrap())
                } else if e.starts_with("add") && e.len() == 4 {
                    Token::Add(e.chars().last().unwrap())
                } else if let Some(label) = e.strip_suffix(':') {
                    Token::Label(label.to_string())
                } else {
                    Token::Ident(e.to_string())
                }
            }
        })
        .collect()
}

pub fn parse(tokens: Vec<Token>) -> Vec<Line> {
    fn line(tokens: &mut impl Iterator<Item = Token>) -> Option<Line> {
        let first = tokens.next()?;

        let (label, first) = match first {
            Token::Label(l) => (Some(l), tokens.next().unwrap()),
            first => (None, first),
        };

        match first {
            Token::Copy | Token::CondJmp(_) | Token::Label(_) => {
                panic!("Unexpected token {first:?}")
            }
            Token::Ident(ident) => match tokens.next().unwrap() {
                Token::CondJmp(c) => {
                    let Some(Token::Ident(target)) = tokens.next() else {
											panic!("Conditional jump requires a label");
									};

                    Some(Line {
                        label,
                        instruction: Instruction::CondJmp(ident, c, target),
                    })
                }
                Token::Copy => {
                    let Some(Token::Ident(reg)) = tokens.next() else {
											panic!("Copying requires a register");
									};

                    Some(Line {
                        label,
                        instruction: Instruction::Copy(ident, reg),
                    })
                }
                tok @ (Token::Label(_)
                | Token::Ident(_)
                | Token::Add(_)
                | Token::Jmp
                | Token::Clr
                | Token::Del
                | Token::Continue) => {
                    panic!("Unexpected token {tok:?}")
                }
            },
            Token::Clr => {
                let Some(Token::Ident(reg)) = tokens.next() else {
									panic!("Clearing requires a register");
							};

                Some(Line {
                    label,
                    instruction: Instruction::Clr(reg),
                })
            }
            Token::Del => {
                let Some(Token::Ident(reg)) = tokens.next() else {
									panic!("Deleting requires a register");
							};

                Some(Line {
                    label,
                    instruction: Instruction::Del(reg),
                })
            }
            Token::Add(c) => {
                let Some(Token::Ident(reg)) = tokens.next() else {
									panic!("Adding requires a register");
							};

                Some(Line {
                    label,
                    instruction: Instruction::Add(reg, c),
                })
            }
            Token::Jmp => {
                let Some(Token::Ident(reg)) = tokens.next() else {
									panic!("Jumping requires a register");
							};

                Some(Line {
                    label,
                    instruction: Instruction::Jmp(reg),
                })
            }
            Token::Continue => Some(Line {
                label,
                instruction: Instruction::Continue,
            }),
        }
    }

    let mut iter = tokens.into_iter();

    let mut res = vec![];
    loop {
        let line = line(&mut iter);

        if let Some(line) = line {
            res.push(line);
        } else {
            break;
        }
    }

    res
}

pub fn run(
    program: Vec<Line>,
    initial_registers: HashMap<String, String>,
) -> HashMap<String, VecDeque<char>> {
    let mut registers = initial_registers
        .into_iter()
        .map(|(reg, val)| (reg, val.chars().collect::<VecDeque<_>>()))
        .collect::<HashMap<_, _>>();
    let jump_table = program
        .iter()
        .enumerate()
        .filter_map(|(i, l)| l.label.clone().map(|l| (l, i)))
        .collect::<HashMap<_, _>>();

    let program = program
        .into_iter()
        .map(|e| e.instruction)
        .collect::<Vec<_>>();

    let mut ip = 0;

    loop {
        match &program[ip] {
            Instruction::Clr(reg) => {
                registers.remove(reg);
            }
            Instruction::Del(reg) => {
                registers
                    .get_mut(reg)
                    .expect("Called del on an empty register")
                    .pop_front()
                    .expect("Called del on an empty register");
            }
            Instruction::Add(reg, c) => {
                if !registers.contains_key(reg) {
                    registers.insert(reg.clone(), VecDeque::new());
                }

                registers.get_mut(reg).unwrap().push_back(*c);
            }
            Instruction::Copy(dest, source) => {
                let content = registers.get(source).cloned().unwrap_or_default();

                registers.insert(dest.clone(), content);
            }
            Instruction::Jmp(label) => {
                ip = *jump_table
                    .get(label)
                    .expect("jmp instruction to unknown label {label}");
                continue;
            }
            Instruction::CondJmp(reg, c, label) => {
                let first = registers.get(reg).and_then(|e| e.get(0));

                if matches!(first, Some(first) if first == c) {
                    ip = *jump_table
                        .get(label)
                        .expect("jmp instruction to unknown label {label}");
                    continue;
                }
            }
            Instruction::Continue => break,
        }

        ip += 1;
    }

    registers
}
