use crate::{Input, Res};

pub fn and<L, R, LT, RT>(l: L, r: R) -> impl Fn(Input) -> Res<(LT, RT)>
where
    L: Fn(Input) -> Res<LT>,
    R: Fn(Input) -> Res<RT>,
{
    move |s| l(s).and_then(|(lt, s)| r(s).and_then(|(rt, s)| Some(((lt, rt), s))))
}
