// SPDX-License-Identifier: Unlicense
// Project: Nyra
// File: Utils/BotConfig.cs
// Authors: Invra

using System;
using System.IO;
using Nyra.Stdout;
using Tomlyn;
using Tomlyn.Model;

namespace Nyra.Config {
  public class BotConfig {
    public string Prefix { get; }
    public string Token { get; }

    private static readonly Lazy<BotConfig> instance = new Lazy<BotConfig>(() => new BotConfig());
    public static BotConfig Config => instance.Value;

    private BotConfig() {
      string configPath = GetConfigLocation();
      bool errors = false;

      if (!File.Exists(configPath)) {
        ConsoleCalls.PrintError($"Config file not found at {configPath}");
        Environment.Exit(1);
      }

      string tomlContent = File.ReadAllText(configPath);
      TomlTable toml;
      try {
        toml = Toml.Parse(tomlContent).ToModel();
      } catch (Exception ex) {
        ConsoleCalls.PrintError($"Failed to parse TOML config: {ex.Message}");
        Environment.Exit(1);
        return;
      }

      var general = toml["general"] as TomlTable;

      Prefix = general?["prefix"]?.ToString() ?? string.Empty;
      if (string.IsNullOrEmpty(Prefix)) {
        ConsoleCalls.PrintError("No 'general.prefix' found in config");
        errors = true;
      } else {
        if (Prefix.Length > 2) {
          ConsoleCalls.PrintWarning("The bot prefix is longer than 2 characters! This may cause impaired usage.");
        }
        ConsoleCalls.PrintStatus($"The provided bot prefix \"{Prefix}\" has been accepted");
      }

      Token = general?["token"]?.ToString()?.Trim() ?? string.Empty;
      if (string.IsNullOrEmpty(Token)) {
        ConsoleCalls.PrintError("No 'general.token' found in config");
        errors = true;
      } else {
        ConsoleCalls.PrintStatus($"The provided Discord token {Token[..10]}… has been accepted");
      }

      if (errors) {
        Environment.Exit(1);
      }
    }

    public static string GetConfigLocation() {
      var envPath = Environment.GetEnvironmentVariable("BOT_CONFIG_PATH");
      if (!string.IsNullOrEmpty(envPath)) {
        ConsoleCalls.PrintStatus($"Using config path from BOT_CONFIG_PATH: {envPath}");
        return envPath;
      }

      string defaultFile = "nyra.toml";
      string defaultDir;

      if (OperatingSystem.IsWindows()) {
        defaultDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "nyra");
      } else {
        defaultDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".config/nyra");
      }

      string path = Path.Combine(defaultDir, defaultFile);
      ConsoleCalls.PrintStatus($"Using default config path: {path}");
      return path;
    }
  }
}
