use crate::{combinators::*, Input, Res};

pub fn digit(radix: u32) -> impl Fn(Input) -> Res<u32> {
    filter_map(any, move |c| c.to_digit(radix))
}
