use crate::{Input, Res};

pub fn or<L, R, T>(l: L, r: R) -> impl Fn(Input) -> Res<T>
where
    L: Fn(Input) -> Res<T>,
    R: Fn(Input) -> Res<T>,
{
    move |s| l(s.clone()).or_else(|| r(s))
}
