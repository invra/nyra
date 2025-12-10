/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: maths/hex.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  num_bigint::BigUint,
  poise::command,
  std::str::FromStr,
};

/// Converts BigUint to Hex
#[command(prefix_command, slash_command, category = "Maths")]
pub async fn hex(
  ctx: Context<'_>,
  #[description = "BigUint to convert"] num: String,
) -> Result<(), Error> {
  let Ok(big) = BigUint::from_str(&num) else {
    ctx
      .say(format!("{num:?} seems to be an invalid number!"))
      .await?;
    return Ok(());
  };

  ctx.say(format!("{big} is `0x{big:x}`.")).await?;

  Ok(())
}

inventory::submit! { MyCommand(hex) }
