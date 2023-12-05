use crate::{Emitter, Instruction};

pub struct Module {
    instructions: Box<[Instruction]>,
}

impl From<Emitter> for Module {
    fn from(value: Emitter) -> Self {
        Self {
            instructions: value.instructions.into_boxed_slice(),
        }
    }
}

impl Module {
    pub fn get(&self, index: usize) -> &Instruction {
        self.instructions.get(index).unwrap_or_else(|| {
            eprintln!("Instruction counter out of bounds.");
            std::process::exit(1);
        })
    }
}
