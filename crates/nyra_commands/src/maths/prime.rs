/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: maths/uuid.rs
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
  std::str::FromStr,
};

fn is_prime(num: BigUint) -> bool {
  let two = BigUint::from(2u32);
  let zero = BigUint::zero();

  if num < two {
    false
  } else if num == two {
    true
  } else if &num % &two == zero {
    false
  } else {
    let mut i = BigUint::from(3u32);
    let limit = num.sqrt();

    while &i <= &limit {
      if &num % &i == zero {
        return false;
      }
      i += &two;
    }

    true
  }
}

/// Validate if BigUint is prime number
#[command(prefix_command, slash_command, category = "Maths")]
pub async fn prime(
  ctx: Context<'_>,
  #[description = "Int to validate"] num: String,
) -> Result<(), Error> {
  let Ok(big) = BigUint::from_str(&num) else {
    ctx
      .say(format!("{num:?} seems to be an invalid number!"))
      .await?;
    return Ok(());
  };

  if is_prime(big) {
    ctx.say(format!("{num} is prime.")).await?;
  } else {
    ctx.say(format!("{num} is not prime.")).await?;
  }

  Ok(())
}

inventory::submit! { MyCommand(prime) }
