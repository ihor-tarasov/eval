use eval_ast::{BinaryOperator, Expression};
use eval_vm::{Emitter, Instruction};

pub fn compile(expression: &Expression, emitter: &mut Emitter) {
    match expression {
        Expression::Integer(value) => emitter.emit(Instruction::Integer(*value)),
        Expression::Binary { first, others } => {
            compile(first, emitter);
            for (operator, expression) in others {
                compile(expression, emitter);
                let instruction = match operator {
                    BinaryOperator::Addict => Instruction::Addict,
                    BinaryOperator::Multiply => Instruction::Multiply,
                };
                emitter.emit(instruction);
            }
        }
    }
}
