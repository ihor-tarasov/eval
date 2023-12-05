use crate::Instruction;

pub struct Emitter {
    pub(crate) instructions: Vec<Instruction>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
