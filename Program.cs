using System.Net.NetworkInformation;

namespace TerryDavis {
  class Program {
    static async Task Main(string[] args) {
      bool ErrState = false;

      // Print non-intrusive warning. As running this as admin
      // in the future is a really dumb idea.
      if (Environment.IsPrivilegedProcess) {
        Console.ForegroundColor = ConsoleColor.Yellow;
        Console.WriteLine("WARNING: This process is running with elevated privileges. Running as an elevated user is not recommended and may pose security risks.");
        Console.ResetColor();
      }

      Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Creating instance");
      DotNetEnv.Env.Load();
      if (Environment.GetEnvironmentVariable("DISCORD_TOKEN") == null) {
        Console.WriteLine($"\x1b[1;31m[STDOUT/critical]:\x1b[0m No DISCORD_TOKEN variable found in the ENV");
        ErrState = true;
      };

      if (Environment.GetEnvironmentVariable("BOT_PREFIX") == null) {
        Console.WriteLine($"\x1b[1;31m[STDOUT/critical]:\x1b[0m No BOT_PREFIX variable found in the ENV");
        ErrState = true;
      };

      if (ErrState) {
        Console.WriteLine($"\x1b[1;31m[STDOUT/critical]:\x1b[0;31m There have been runtime errors. Please solve them to be able to continue\x1b[0m");
        return;
      }

      Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Testing network connectivity to Discord");
      try {
        Ping myPing = new Ping();
        String host = "discord.com";
        byte[] buffer = new byte[32];
        int timeout = 1000;
        PingOptions pingOptions = new PingOptions();
        PingReply reply = myPing.Send(host, timeout, buffer, pingOptions);
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Networking test has passed");
      } catch {
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Cannot contact Discord, this is just a warning it may not connect.");
      }

      var bot = new Bot();
      await bot.RunAsync();
    }
  }
}
