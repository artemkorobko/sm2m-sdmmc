use crate::error::AppError;

use super::Mode;

pub enum Complete {
    Continue,
    Mode(Mode),
    Reply(u16),
}

pub type ModeResult = core::result::Result<Complete, AppError>;
