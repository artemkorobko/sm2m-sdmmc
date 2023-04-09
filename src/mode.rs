use alloc::{string::String, vec::Vec};

use crate::error::AppError;

pub enum Mode {
    Ready,
    Address(String),
    Write(String, Vec<u8>),
    Read(String, Vec<u8>, usize),
    Error(AppError),
}
