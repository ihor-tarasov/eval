use crate::{Input, Res};

pub fn filter<T, P, F>(p: P, f: F) -> impl Fn(Input) -> Res<T>
where
    P: Fn(Input) -> Res<T>,
    F: Fn(&T) -> bool,
{
    move |s| p(s).and_then(|(t, s)| if f(&t) { Some((t, s)) } else { None })
}
