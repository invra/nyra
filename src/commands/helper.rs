/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  File: commands/helper.rs
 *  Authors: Invra, Hiten-Tandon
 */

use poise::Command;

#[derive(Debug)]
pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct MyCommand(pub fn() -> Command<Data, Error>);
