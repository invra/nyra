/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_config
 *  File: path.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::model::*,
  nyra_utils::log,
};

pub trait Validate {
  fn validate(&self, errors: &mut Vec<String>);
}

impl Validate for Config {
  fn validate(&self, errors: &mut Vec<String>) {
    self.general.validate(errors);
  }
}

impl Validate for General {
  fn validate(&self, errors: &mut Vec<String>) {
    if self.token.trim().is_empty() {
      errors.push("Discord bot token is missing or empty".into());
    }

    let prefix = self.prefix.clone().unwrap_or_default();

    if prefix.trim().is_empty() {
      errors.push("Command prefix is missing or empty".into());
    }

    if prefix.chars().count() > 2 {
      log::warning!("Prefix length > 2 characters may impair usage");
    }
  }
}
