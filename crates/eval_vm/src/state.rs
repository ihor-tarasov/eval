use crate::{Instruction, Module};

pub struct State {
    stack: Vec<i64>,
    instruction_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            instruction_index: 0,
        }
    }

    fn push(&mut self, value: i64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or_else(|| {
            eprintln!("Stack underflow");
            std::process::exit(1);
        })
    }

    pub fn exec(&mut self, module: &Module) -> i64 {
        loop {
            let instruction = module.get(self.instruction_index);
            self.instruction_index += 1;
            match instruction {
                Instruction::Integer(value) => self.push(*value),
                Instruction::Addict => {
                    let right = self.pop();
                    let left = self.pop();
                    self.push(left.wrapping_add(right));
                }
                Instruction::Multiply => {
                    let right = self.pop();
                    let left = self.pop();
                    self.push(left.wrapping_mul(right));
                }
                Instruction::End => break self.pop(),
            }
        }
    }
}
