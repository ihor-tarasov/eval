use std::str::Chars;

type Res<'a, T> = Option<(T, Chars<'a>)>;

// Combinators

fn any(mut s: Chars) -> Res<char> {
    Some((s.next()?, s))
}

fn filter<T, P, F>(p: P, f: F) -> impl Fn(Chars) -> Res<T>
where
    P: Fn(Chars) -> Res<T>,
    F: Fn(&T) -> bool,
{
    move |s| p(s).and_then(|(t, s)| if f(&t) { Some((t, s)) } else { None })
}

fn map<T, E, P, F>(p: P, f: F) -> impl Fn(Chars) -> Res<E>
where
    P: Fn(Chars) -> Res<T>,
    F: Fn(T) -> E,
{
    move |s| p(s).and_then(|(t, s)| Some((f(t), s)))
}

fn filter_map<T, E, P, F>(p: P, f: F) -> impl Fn(Chars) -> Res<E>
where
    P: Fn(Chars) -> Res<T>,
    F: Fn(T) -> Option<E>,
{
    move |s| p(s).and_then(|(t, s)| Some((f(t)?, s)))
}

fn and<L, R, LT, RT>(l: L, r: R) -> impl Fn(Chars) -> Res<(LT, RT)>
where
    L: Fn(Chars) -> Res<LT>,
    R: Fn(Chars) -> Res<RT>,
{
    move |s| l(s).and_then(|(lt, s)| r(s).and_then(|(rt, s)| Some(((lt, rt), s))))
}

fn or<L, R, T>(l: L, r: R) -> impl Fn(Chars) -> Res<T>
where
    L: Fn(Chars) -> Res<T>,
    R: Fn(Chars) -> Res<T>,
{
    move |s| l(s.clone()).or_else(|| r(s))
}

// Utils

fn digit(radix: u32) -> impl Fn(Chars) -> Res<u32> {
    filter_map(any, move |c| c.to_digit(radix))
}

// VM

enum Instruction {
    Integer(i64),
    Addict,
    Multiply,
    End,
}

struct Emitter {
    instructions: Vec<Instruction>,
}

impl Emitter {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

struct Module {
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
    fn get(&self, index: usize) -> &Instruction {
        self.instructions.get(index).unwrap_or_else(|| {
            eprintln!("Instruction counter out of bounds.");
            std::process::exit(1);
        })
    }
}

struct State {
    stack: Vec<i64>,
    instruction_index: usize,
}

impl State {
    fn new() -> Self {
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

    fn exec(&mut self, module: &Module) -> i64 {
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

// Expression

#[derive(Clone, Copy)]
enum BinaryOperator {
    Addict,
    Multiply,
}

enum Expression {
    Integer(i64),
    Binary {
        first: Box<Expression>,
        others: Vec<(BinaryOperator, Expression)>,
    },
}

impl Expression {
    fn compile(&self, emitter: &mut Emitter) {
        match self {
            Expression::Integer(value) => emitter.emit(Instruction::Integer(*value)),
            Expression::Binary { first, others } => {
                first.compile(emitter);
                for (operator, expression) in others {
                    expression.compile(emitter);
                    let instruction = match operator {
                        BinaryOperator::Addict => Instruction::Addict,
                        BinaryOperator::Multiply => Instruction::Multiply,
                    };
                    emitter.emit(instruction);
                }
            }
        }
    }
}

// Parser

fn integer() -> impl Fn(Chars) -> Res<Expression> {
    map(digit(10), |d| Expression::Integer(d as i64))
}

fn primary() -> impl Fn(Chars) -> Res<Expression> {
    integer()
}

fn factor_operator() -> impl Fn(Chars) -> Res<BinaryOperator> {
    filter_map(any, |c| match c {
        '*' => Some(BinaryOperator::Multiply),
        _ => None,
    })
}

fn term_operator() -> impl Fn(Chars) -> Res<BinaryOperator> {
    filter_map(any, |c| match c {
        '+' => Some(BinaryOperator::Addict),
        _ => None,
    })
}

fn binary<O, N>(o: O, n: N) -> impl Fn(Chars) -> Res<Expression>
where
    O: Fn(Chars) -> Res<BinaryOperator>,
    N: Fn(Chars) -> Res<Expression>,
{
    move |s| {
        let (first, mut s) = n(s)?;
        let mut others = Vec::new();
        while let Some((operator, new_s)) = o(s.clone()) {
            match n(new_s) {
                Some((right, new_s)) => {
                    others.push((operator, right));
                    s = new_s;
                }
                None => {
                    panic!("Expected expression.")
                }
            }
        }
        Some((
            if others.is_empty() {
                first
            } else {
                Expression::Binary {
                    first: Box::new(first),
                    others,
                }
            },
            s,
        ))
    }
}

fn factor() -> impl Fn(Chars) -> Res<Expression> {
    binary(factor_operator(), primary())
}

fn term() -> impl Fn(Chars) -> Res<Expression> {
    binary(term_operator(), factor())
}

fn expression() -> impl Fn(Chars) -> Res<Expression> {
    term()
}

fn parse(code: &str) -> Expression {
    let parser = expression();
    let (expression, mut rest) = parser(code.chars()).unwrap();
    if let Some(c) = rest.next() {
        eprintln!("Expected end of file, found '{c}'");
        std::process::exit(1);
    }
    expression
}

fn compile(expression: &Expression) -> Module {
    let mut emitter = Emitter::new();
    expression.compile(&mut emitter);
    emitter.emit(Instruction::End);
    emitter.into()
}

fn exec(module: &Module) -> i64 {
    let mut state = State::new();
    state.exec(module)
}

fn build(code: &str) -> Module {
    compile(&parse(code))
}

trait Eval {
    fn eval(&self) -> i64;
}

impl Eval for &str {
    fn eval(&self) -> i64 {
        exec(&build(self))
    }
}

fn main() {
    println!("{}", "2+2*2".eval());
}
