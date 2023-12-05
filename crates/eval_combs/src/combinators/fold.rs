use crate::{Input, Res};

pub fn fold<P, I, F, T, E>(p: P, init: I, f: F) -> impl Fn(Input) -> Res<E>
where
    P: Fn(Input) -> Res<T>,
    I: Fn() -> E,
    F: Fn(E, T) -> E,
{
    move |mut i| {
        let mut e = init();
        while let Some((t, new_i)) = p(i.clone()) {
            e = f(e, t);
            i = new_i;
        }
        Some((e, i))
    }
}
