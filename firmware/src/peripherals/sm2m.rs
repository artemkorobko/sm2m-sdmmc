pub mod bus;
pub mod input;
pub mod output;

mod gpioa;
mod gpiob;
mod gpioc;
mod gpiod;
mod gpioe;
mod io;

pub use input::{Command, Input, InputBus, InvertedInputBus, Read};
pub use output::{InvertedOutputBus, Output, OutputBus, Write};

pub mod prelude {
    pub(crate) use super::gpioa::SM2MGPIOAMap;
    pub(crate) use super::gpiob::SM2MGPIOBMap;
    pub(crate) use super::gpioc::SM2MGPIOCMap;
    pub(crate) use super::gpiod::SM2MGPIODMap;
    pub(crate) use super::gpioe::SM2MGPIOEMap;
}
