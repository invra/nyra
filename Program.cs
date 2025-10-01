/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Program.cs
    Authors: Invra
    Notes: Entrypoint file
*/

using System.Net.NetworkInformation;
using Nyra.Colourise;
using Nyra.Stdout;

namespace Nyra {
  class Program {
    static async Task Main(string[] args) {
      ConsoleCalls.PrintStatus("Creating instance");
      ConsoleCalls.PrintStatus("Testing network connectivity to Discord");

      try {
        Ping myPing = new Ping();
        String host = "discord.com";
        byte[] buffer = new byte[32];
        int timeout = 1000;
        PingOptions pingOptions = new PingOptions();
        PingReply reply = myPing.Send(host, timeout, buffer, pingOptions);
        ConsoleCalls.PrintStatus("Networking test has passed");
      } catch {
        ConsoleCalls.PrintWarning("Cannot contact Discord, this is just a warning it may not connect.");
      }

      await Bot.Nyra.RunAsync();
    }
  }
}
