/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

pub(crate) mod helper;
pub(crate) mod information;

use helper::*;

inventory::collect!(MyCommand);

#[inline(always)]
pub fn all() -> Vec<poise::Command<Data, Error>> {
  inventory::iter::<MyCommand>().map(|x| (x.0)()).collect()
}
