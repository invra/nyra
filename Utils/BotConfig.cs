/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/BotConfig.cs
    Authors: Invra
    Notes: Singleton for Config
*/

using Nyra.Stdout;

namespace Nyra.Config {
  public class BotConfig {
    public string Prefix { get; }
    public string Token { get; }

    private static readonly Lazy<BotConfig> instance = new Lazy<BotConfig>(() => new BotConfig());
    public static BotConfig Config => instance.Value;

    private BotConfig() {
      DotNetEnv.Env.Load();
      bool errors = false;

      Prefix = Environment.GetEnvironmentVariable("BOT_PREFIX") ?? string.Empty;
      if (string.IsNullOrEmpty(Prefix)) {
        ConsoleCalls.PrintError("No BOT_PREFIX variable found in the ENV");
        errors = true;
      } else {
        if (Prefix.Length > 2) {
          ConsoleCalls.PrintWarning("The bot prefix is longer than 2 characters! This may cause impaired usage.");
        }
        ConsoleCalls.PrintStatus($"The provided bot prefix \"{Prefix}\" has been accepted");
      }

      Token = Environment.GetEnvironmentVariable("DISCORD_TOKEN")?.Trim() ?? string.Empty;
      if (string.IsNullOrEmpty(Token)) {
        ConsoleCalls.PrintError("No DISCORD_TOKEN variable found in the ENV");
        errors = true;
      } else {
        ConsoleCalls.PrintStatus($"The provided Discord token {Token[..10]}… has been accepted");
      }

      if (errors) {
        Environment.Exit(1);
      }
    }
  }
}
