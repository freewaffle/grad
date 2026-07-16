#![allow(dead_code)]

pub const REGISTER_COUNT: usize = 128;
pub const MAX_ROUTINE_COUNT: usize = 256;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Nil,
    Number(f32),
    Boolean(bool),
    Address(usize)
}
