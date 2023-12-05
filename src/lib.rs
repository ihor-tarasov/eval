use eval_ast::Expression;
use eval_vm::{Module, Emitter, Instruction, State};


pub fn parse(code: &str) -> Expression {
    let parser = eval_parser::parser();
    let (expression, mut rest) = parser(code.chars()).unwrap();
    if let Some(c) = rest.next() {
        eprintln!("Expected end of file, found '{c}'");
        std::process::exit(1);
    }
    expression
}

pub fn compile(expression: &Expression) -> Module {
    let mut emitter = Emitter::new();
    eval_compiler::compile(expression, &mut emitter);
    emitter.emit(Instruction::End);
    emitter.into()
}

pub fn exec(module: &Module) -> i64 {
    let mut state = State::new();
    state.exec(module)
}

pub fn build(code: &str) -> Module {
    compile(&parse(code))
}

pub trait Eval {
    fn eval(&self) -> i64;
}

impl Eval for &str {
    fn eval(&self) -> i64 {
        exec(&build(self))
    }
}
