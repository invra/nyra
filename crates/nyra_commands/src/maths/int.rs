/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: maths/random.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  num_bigint::BigUint,
  num_traits::Zero,
  poise::command,
  rand::Rng,
  std::str::FromStr,
};

/// Random integer gen
#[command(prefix_command, slash_command, category = "Maths")]
pub async fn int(
  ctx: Context<'_>,
  #[description = "Range"] num: Option<String>,
) -> Result<(), Error> {
  let Ok(max) = BigUint::from_str(num.as_deref().unwrap_or("100")) else {
    ctx
      .say(format!(
        "{:?} is not a valid number!",
        num.unwrap_or("100".into()),
      ))
      .await?;
    return Ok(());
  };

  if max.is_zero() {
    ctx.say("Max cannot be zero!").await?;
    return Ok(());
  }

  let byte_len = max.to_bytes_be().len();
  let mut bytes = vec![0u8; byte_len];

  ctx
    .say({
      let mut rng = rand::thread_rng();

      let n = loop {
        rng.fill(&mut bytes[..]);
        let candidate = BigUint::from_bytes_be(&bytes);

        if candidate <= max {
          break candidate;
        }
      };

      format!("Generated random BigUint (0–{}): `{}`", max, n)
    })
    .await?;

  Ok(())
}

inventory::submit! { MyCommand(int) }
