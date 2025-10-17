/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  File: commands/helper.rs
 *  Authors: Invra, Hiten-Tandon
 */

use poise::Command;

#[derive(Debug)]
pub(crate) struct Data;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
pub(crate) struct MyCommand(pub fn() -> Command<Data, Error>);
