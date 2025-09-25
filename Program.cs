using System.Net.NetworkInformation;

namespace Nyra {
  class Program {
    static async Task Main(string[] args) {
      Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Creating instance");
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

      await Bot.Nyra.RunAsync();
    }
  }
}
