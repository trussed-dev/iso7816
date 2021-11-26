#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate delog;
// generate_macros!();

#[derive(Copy, Clone, PartialEq)]
pub enum Interface {
    Contact,
    Contactless,
}

pub type Result<T=()> = core::result::Result<T, Status>;

pub mod aid;
pub mod command;
pub mod response;
pub mod somebytes;

pub use aid::{Aid, App};
pub use command::{Command, Instruction};
pub use response::{Response, Status};
