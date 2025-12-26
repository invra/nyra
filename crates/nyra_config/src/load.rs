/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_config
 *  File: path.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::{
    Config,
    ConfigError,
    path,
    validate::Validate,
  },
  nyra_utils::log,
  std::{
    fs,
    path::PathBuf,
  },
};
pub fn load(config: Option<String>) -> Result<Config, ConfigError> {
  let path: PathBuf = config.map_or_else(path::config_path, PathBuf::from);

  log::info!("Loading config from: {}", path.display());

  if !path.exists() {
    return Err(ConfigError::FileNotFound(path));
  }

  let content = fs::read_to_string(&path).map_err(ConfigError::ReadError)?;
  let config: Config =
    toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

  let mut errors = Vec::new();
  config.validate(&mut errors);

  if !errors.is_empty() {
    return Err(ConfigError::ValidationErrors(errors));
  }

  Ok(config)
}
