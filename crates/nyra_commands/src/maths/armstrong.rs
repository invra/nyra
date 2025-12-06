/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: utilities/ping.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  num_bigint::BigUint,
  num_traits::{
    One,
    Zero,
  },
  poise::command,
  std::str::FromStr,
};

/// I dont know how to do proper logic so im extracting it
fn is_armstrong(num: BigUint) -> bool {
  let digits: Vec<u32> = num
    .to_str_radix(10)
    .chars()
    .map(|d| d.to_digit(10).unwrap())
    .collect();

  let len = digits.len() as u32;
  let mut sum = BigUint::zero();

  for d in digits {
    let mut pow = BigUint::one();
    let big_d = BigUint::from(d);
    for _ in 0..len {
      pow *= &big_d;
    }
    sum += pow;
  }

  sum == num
}

/// Validates if given BigUnit is an armstrong
#[command(prefix_command, slash_command, category = "Maths")]
pub async fn armstrong(
  ctx: Context<'_>,
  #[description = "User to check"] num: String,
) -> Result<(), Error> {
  let Ok(big) = BigUint::from_str(&num) else {
    ctx
      .say(format!("{num:?} seem to be an invalid number!"))
      .await?;
    return Ok(());
  };

  if is_armstrong(big) {
    ctx.say(format!("{num} is an armstrong.")).await?;
  } else {
    ctx.say(format!("{num} is not an armstrong.")).await?;
  }

  Ok(())
}

inventory::submit! { MyCommand(armstrong) }
