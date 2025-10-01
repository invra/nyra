using System.Net.NetworkInformation;
using Nyra.Colourise;

namespace Nyra {
  class Program {
    static async Task Main(string[] args) {
      Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Creating instance");
      Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Testing network connectivity to Discord");
      try {
        Ping myPing = new Ping();
        String host = "discord.com";
        byte[] buffer = new byte[32];
        int timeout = 1000;
        PingOptions pingOptions = new PingOptions();
        PingReply reply = myPing.Send(host, timeout, buffer, pingOptions);
        Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Networking test has passed");
      } catch {
        Console.WriteLine($"{"[STDOUT/warning]:".Yellow().Bold()} Cannot contact Discord, this is just a warning it may not connect.");
      }

      await Bot.Nyra.RunAsync();
    }
  }
}
