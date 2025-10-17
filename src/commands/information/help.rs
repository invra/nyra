/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/help.rs
  Authors: Invra, Hiten-Tandon
*/

use {
  crate::commands::helper::{Context, Error, MyCommand},
  poise::{
    builtins::{self, HelpConfiguration},
    command,
  },
};

/// Show this menu
#[command(prefix_command, track_edits, slash_command, category = "information")]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
  let config = HelpConfiguration::default();
  builtins::help(ctx, command.as_deref(), config)
    .await
    .map_err(Into::into)
}
inventory::submit! { MyCommand(help) }
