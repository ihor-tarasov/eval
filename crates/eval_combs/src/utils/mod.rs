use crate::{combinators::*, Input, Res};

pub fn digit(radix: u32) -> impl Fn(Input) -> Res<u32> {
    filter_map(any, move |c| c.to_digit(radix))
}

fn merge_integer(i: u64, d: u32, radix: u32) -> u64 {
    i * radix as u64 + d as u64
}

pub fn integer(radix: u32) -> impl Fn(Input) -> Res<u64> {
    filter_map(
        fold(
            digit(radix),
            || None,
            move |a, d| Some(merge_integer(a.unwrap_or(0), d, radix)),
        ),
        |o| o,
    )
}
