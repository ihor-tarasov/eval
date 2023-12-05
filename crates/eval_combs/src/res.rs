use crate::Input;

pub type Res<'a, T> = Option<(T, Input<'a>)>;
