use crate::{Input, Res};

pub fn filter_map<T, E, P, F>(p: P, f: F) -> impl Fn(Input) -> Res<E>
where
    P: Fn(Input) -> Res<T>,
    F: Fn(T) -> Option<E>,
{
    move |s| p(s).and_then(|(t, s)| Some((f(t)?, s)))
}
