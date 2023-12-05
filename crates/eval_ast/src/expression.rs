use crate::BinaryOperator;

pub enum Expression {
    Integer(i64),
    Binary {
        first: Box<Expression>,
        others: Vec<(BinaryOperator, Expression)>,
    },
}
