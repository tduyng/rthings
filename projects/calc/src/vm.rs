use std::collections::HashMap;

use crate::{alloc, types::Value};

struct Stackframe<'a> {
    pub variables: HashMap<String, Value>,
    pub parent: Option<&'a Stackframe<'a>>,
}

impl<'a> Stackframe<'a> {
    pub fn search(&self, variable: String) -> Option<Value> {
        if let Some(val) = self.variables.get(&variable) {
            return Some(val.clone());
        } else if let Some(frame) = self.parent {
            return frame.search(variable);
        }
        None
    }
}

pub struct Vm<'a> {
    registers: Vec<Option<Value>>,
    constants: Vec<Value>,
    instructions: Vec<Operation>,
    frame: Stackframe<'a>,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Neg,
    Load,
    StoreLocal,
    LoadLocal,
    Store,
    Debug,
    Argument(usize),
}

impl<'a> Vm<'a> {
    pub fn new(c: &alloc::Pool, instructions: Vec<Operation>) -> Vm {
        Vm {
            registers: vec![None; 128],
            instructions,
            constants: c.constants.clone(),
            frame: Stackframe {
                variables: HashMap::new(),
                parent: None,
            },
        }
    }

    pub fn run(&mut self) {
        if self.instructions.len() % 2 != 0 {
            panic!("Instruction array is invalid");
        }

        for instr_pair in self.instructions.chunks_exact(2) {
            let operation = instr_pair[0];
            let argument = match instr_pair[1] {
                Operation::Argument(v) => v,
                _ => panic!("Wanted an operation of type Argument, got something else"),
            };

            match operation {
                Operation::LoadLocal => {
                    let name = self
                        .constants
                        .get(argument)
                        .unwrap_or_else(|| panic!("Wanted constant at index {}", argument));
                    if let Value::Ident(ident) = name {
                        self.registers[0] = self.frame.search(ident.to_string());
                    } else {
                        panic!("Ident somehow isn't an ident, this should be impossible")
                    }
                }
                Operation::StoreLocal => {
                    let r0 = self.registers[0]
                        .clone()
                        .expect("Failed to get anything from r0");
                    let name = self
                        .constants
                        .get(argument)
                        .unwrap_or_else(|| panic!("Wanted constant at index {}", argument));
                    if let Value::Ident(ident) = name {
                        self.frame.variables.insert(ident.to_string(), r0);
                        self.registers[0] = None;
                    } else {
                        panic!("Ident somehow isn't an ident, this should be impossible")
                    }
                }
                Operation::Load => {
                    let constant = self
                        .constants
                        .get(argument)
                        .unwrap_or_else(|| panic!("Wanted constant at index {}", argument))
                        .clone();
                    self.registers[0] = Some(constant);
                }
                Operation::Store => {
                    let val = self.registers[0].clone();
                    self.registers[argument] = val;
                    self.registers[0] = None;
                }
                Operation::Add | Operation::Sub | Operation::Div | Operation::Mul => {
                    let first = self.registers[argument]
                        .clone()
                        .unwrap_or_else(|| panic!("Invalid register at index {}", argument));
                    let second = self.registers[0].clone().expect("r0 holds no value");

                    let r = match operation {
                        Operation::Add => first.add(second),
                        Operation::Sub => first.sub(second),
                        Operation::Mul => first.mul(second),
                        Operation::Div => first.div(second),
                        _ => panic!("Not supported"),
                    };

                    if r.is_some() {
                        self.registers[0] = r;
                    } else {
                        panic!(
                            "Can't perform Operation::{:#?} on {:?} and {:?}",
                            operation, first, second
                        );
                    }
                }
                Operation::Neg => {
                    let first = self.registers[argument]
                        .clone()
                        .unwrap_or_else(|| panic!("Invalid register at index {}", argument));
                    self.registers[0] = first.mul(Value::Number(-1.0));
                }
                Operation::Debug => {
                    println!(
                        "Operation::Debug at r{}: {:?}",
                        argument,
                        *self
                            .registers
                            .get(argument)
                            .unwrap_or_else(|| panic!("Invalid register at index {}", argument))
                    )
                }
                o => panic!("Operation::{:?} not implemented", o),
            }
        }
    }
}
