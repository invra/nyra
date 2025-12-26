/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_config
 *  File: model.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  serde::Deserialize,
  std::ops::Deref,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub general: General,
  pub db: Option<DatabaseOpts>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
  pub token: String,
  pub prefix: Option<Prefix>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prefix(String);

impl Deref for Prefix {
  type Target = str;
  fn deref(&self) -> &str {
    &self.0
  }
}

impl Default for Prefix {
  fn default() -> Self {
    Self("? ".into())
  }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DatabaseOpts {
  pub host: Option<Host>,
  pub port: Option<Port>,
  pub username: Option<Username>,
  pub password: Option<Password>,
}

macro_rules! string_newtype {
  ($name:ident, $default:expr) => {
    #[derive(Debug, Deserialize, Clone)]
    pub struct $name(String);

    impl Default for $name {
      fn default() -> Self {
        Self($default.into())
      }
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
      }
    }
  };
}

string_newtype!(Host, "127.0.0.1");
string_newtype!(Username, "mongodb");
string_newtype!(Password, "mongo");

#[derive(Debug, Deserialize, Clone)]
pub struct Port(u16);

impl Default for Port {
  fn default() -> Self {
    Self(27017)
  }
}

impl std::fmt::Display for Port {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}
