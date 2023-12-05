use crate::{Input, Res};

pub fn any(mut i: Input) -> Res<char> {
    Some((i.next()?, i))
}
