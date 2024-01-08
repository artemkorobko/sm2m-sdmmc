pub mod address;
pub mod read;
pub mod ready;
pub mod result;
pub mod write;

use alloc::{string::String, vec::Vec};

use crate::error::AppError;

pub enum Mode {
    Ready,
    Address(String),
    Write(String, Vec<u8>),
    Read(String, u32, Vec<u8>, usize),
    Error(AppError),
}
