use eval_ast::{BinaryOperator, Expression};
use eval_combs::{combinators::*, utils::*, Input, Res};

pub fn integer() -> impl Fn(Input) -> Res<Expression> {
    map(digit(10), |d| Expression::Integer(d as i64))
}

pub fn primary() -> impl Fn(Input) -> Res<Expression> {
    integer()
}

fn factor_operator() -> impl Fn(Input) -> Res<BinaryOperator> {
    filter_map(any, |c| match c {
        '*' => Some(BinaryOperator::Multiply),
        _ => None,
    })
}

fn term_operator() -> impl Fn(Input) -> Res<BinaryOperator> {
    filter_map(any, |c| match c {
        '+' => Some(BinaryOperator::Addict),
        _ => None,
    })
}

fn binary<O, N>(o: O, n: N) -> impl Fn(Input) -> Res<Expression>
where
    O: Fn(Input) -> Res<BinaryOperator>,
    N: Fn(Input) -> Res<Expression>,
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

pub fn factor() -> impl Fn(Input) -> Res<Expression> {
    binary(factor_operator(), primary())
}

pub fn term() -> impl Fn(Input) -> Res<Expression> {
    binary(term_operator(), factor())
}

pub fn parser() -> impl Fn(Input) -> Res<Expression> {
    term()
}
