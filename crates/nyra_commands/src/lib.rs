/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

pub mod helper;
pub mod information;
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
