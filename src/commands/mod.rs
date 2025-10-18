/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

pub(crate) mod helper;
pub(crate) mod information;
pub(crate) mod moderation;
pub(crate) mod utilities;

use {
  crate::commands::helper::{Data, Error, MyCommand},
  poise::Command,
};

inventory::collect!(MyCommand);

#[inline(always)]
pub fn all() -> Vec<Command<Data, Error>> {
  inventory::iter::<MyCommand>().map(|x| (x.0)()).collect()
}
