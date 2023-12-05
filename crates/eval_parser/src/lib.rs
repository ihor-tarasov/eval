use eval_ast::Expression;
use eval_combs::{Input, Res};

pub mod expression;

pub fn parser() -> impl Fn(Input) -> Res<Expression> {
    expression::parser()
}
