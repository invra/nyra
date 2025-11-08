/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: config.rs
    Authors: Invra
    Notes: Configuration handling

    Config file location:
    - Unix/macOS: ~/.config/nyra/nyra.toml
    - Windows: %APPDATA%\nyra\nyra.toml
*/

use {
  nyra_utils::log,
  serde::Deserialize,
  std::{
    fs,
    path::PathBuf,
    str::FromStr,
  },
};

#[derive(Debug)]
pub enum ConfigError {
  FileNotFound(PathBuf),
  ReadError(std::io::Error),
  ParseError(String),
  ValidationErrors(Vec<String>),
}

impl std::fmt::Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::FileNotFound(path) => {
        write!(f, "Config file not found at: {}.", path.display(),)
      }
      Self::ReadError(e) => write!(f, "Failed to read config file: {e}"),
      Self::ParseError(e) => write!(f, "Failed to parse TOML: {e}"),
      Self::ValidationErrors(errors) => {
        writeln!(f, "Config validation failed:")?;
        for error in errors {
          writeln!(f, "- {error}")?;
        }
        Ok(())
      }
    }
  }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
  pub general: General,
}

#[derive(Deserialize, Clone, Debug)]
pub struct General {
  pub token: String,
  pub prefix: String,
}

impl Config {
  pub fn load(config: Option<String>) -> Result<Self, ConfigError> {
    let config_path = config.map_or_else(Self::get_config_path, PathBuf::from);

    Self::load_from_path(&config_path)
  }

  pub fn load_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
    let config_path = path.as_ref();
    log::info!("Loading config from: {}", config_path.display());

    if !config_path.exists() {
      return Err(ConfigError::FileNotFound(config_path.to_path_buf()));
    }

    let fs_str = fs::read_to_string(config_path).map_err(ConfigError::ReadError)?;

    let config: Self = match toml::from_str(&fs_str) {
      Ok(config) => config,
      Err(e) => {
        let mut validation_errors = Vec::new();
        let err_str = e.to_string();

        if err_str.contains("missing field") {
          if err_str.contains("missing field `token`") {
            validation_errors.push("Discord bot token is missing".to_string());
          }
          if err_str.contains("missing field `prefix`") {
            validation_errors.push("Command prefix is missing".to_string());
          }
          if err_str.contains("missing field `general`") {
            validation_errors.push("The [general] section is missing".to_string());
          }
          return Err(ConfigError::ValidationErrors(validation_errors));
        }

        let error_msg = if err_str.contains("expected `=`") {
          "Invalid syntax: missing '=' after key".to_string()
        } else if err_str.contains("unexpected `=`") {
          "Invalid syntax: unexpected '=' found".to_string()
        } else if err_str.contains("expected a table key") {
          "Invalid syntax: expected a key name".to_string()
        } else if err_str.contains("unquoted string") {
          "Invalid syntax: string value must be quoted".to_string()
        } else {
          format!("Invalid TOML syntax: {e}")
        };

        return Err(ConfigError::ParseError(error_msg));
      }
    };

    let mut validation_errors = Vec::new();

    if config.general.token.trim().is_empty() {
      validation_errors.push("Discord bot token is missing or empty".to_string());
    }

    if config.general.prefix.trim().is_empty() {
      validation_errors.push("Command prefix is missing or empty".to_string());
    }

    if config.general.prefix.chars().count() > 2 {
      log::warning!(
        "The prefix length is over 2 characters, which can cause impaired usage with the bot.",
      );
    }

    if !validation_errors.is_empty() {
      return Err(ConfigError::ValidationErrors(validation_errors));
    }

    Ok(config)
  }

  pub fn get_config_path() -> PathBuf {
    if cfg!(unix) {
      std::env::home_dir()
        .unwrap()
        .join(".config")
        .join("nyra")
        .join("nyra.toml")
    } else {
      PathBuf::from_str(&std::env::var("LOCALAPPDATA").unwrap())
        .unwrap()
        .join("nyra")
        .join("nyra.toml")
    }
  }
}
