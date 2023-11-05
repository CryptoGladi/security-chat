//! This crate is responsible for the start `command`
//!
//! The command is a tool to speed up the user's work
//!
//! Allows you to save time and not use the mouse. Inspired by the [VIM hotkeys](`https://vim.rtorr.com/`)
//!
//! Main module is [`crate::runner`]

pub mod command;
pub mod prelude;
pub mod runner;

#[cfg(test)]
pub(crate) mod test_utils;
