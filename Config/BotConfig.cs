namespace TerryDavis.Config {
  public class BotConfig {
    public string Prefix { get; }
    public string Token { get; }

    private static readonly Lazy<BotConfig> instance = new Lazy<BotConfig>(() => new BotConfig());
    public static BotConfig Config { get { return instance.Value; } }

    private BotConfig() {
      DotNetEnv.Env.Load();
      bool errors = false;

      try {
        Prefix = Environment.GetEnvironmentVariable("BOT_PREFIX")!;
        if(Prefix.Length > 2) {
          Console.WriteLine("\x1b[0;33m[STDOUT/WARNING]: The bot prefix is longer than 2 characters! This will cause impaired usage.\x1b[0;0m");
        }
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m \"{Prefix}\" has been accepted as the bot prefix");
      } catch {
        Prefix = string.Empty;
        Console.WriteLine($"\x1b[1;31m[STDERR/critical]:\x1b[0m No BOT_PREFIX variable found in the ENV", Console.Error);
        errors = true;
      }

      try {
        Token = Environment.GetEnvironmentVariable("DISCORD_TOKEN")!;
        Token = (Token.Trim() == string.Empty ? null : Token)!;
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Provided Discord token {Token[..10]}… has been accepted");
      } catch {
        Token = string.Empty;
        Console.WriteLine($"\x1b[1;31m[STDERR/critical]:\x1b[0m No DISCORD_TOKEN variable found in the ENV", Console.Error);
        errors = true;
      }

      if (errors) {
        Environment.Exit(1);
      }
    }
  }
}
