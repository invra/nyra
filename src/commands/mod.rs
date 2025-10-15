/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

pub(crate) mod helper;
pub(crate) mod information;

use {
  helper::{
    Data,
    Error,
  },
  poise::Command,
};

pub(crate) struct MyCommand(fn() -> Command<Data, Error>);
inventory::collect!(MyCommand);

#[inline(always)]
pub fn all() -> Vec<Command<Data, Error>> {
  inventory::iter::<MyCommand>().map(|x| (x.0)()).collect()
}
