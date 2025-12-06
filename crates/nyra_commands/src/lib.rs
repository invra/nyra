/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: lib.rs
 *  Authors: Invra
 */

pub mod helper;
pub mod information;
pub mod maths;
pub mod moderation;
pub mod utilities;

use {
  crate::helper::{
    Data,
    Error,
    MyCommand,
  },
  poise::Command,
};

inventory::collect!(MyCommand);

pub fn all() -> Vec<Command<Data, Error>> {
  inventory::iter::<MyCommand>().map(|x| (x.0)()).collect()
}
