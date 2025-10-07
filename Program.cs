/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Program.cs
    Authors: Invra
    Notes: Entrypoint file
*/

using System.CommandLine;
using System.CommandLine.Invocation;
using System.Net.NetworkInformation;
using Nyra.Colourise;
using Nyra.Stdout;

namespace Nyra {
  class Program {
    static int Main(string[] args) {
      var configOption = new Option<string>(
          aliases: new[] { "--config", "-c" },
          description: "Path to the configuration file") {
        IsRequired = false
      };

      var rootCommand = new RootCommand("Nyra Discord bot");
      rootCommand.Add(configOption);
      rootCommand.Handler = new CommandHandlerImpl(configOption);

      return rootCommand.Invoke(args);
    }

    private class CommandHandlerImpl : ICommandHandler {
      private readonly Option<string> configOption;
      public CommandHandlerImpl(Option<string> configOption) => this.configOption = configOption;

      public int Invoke(InvocationContext context) { return Task.Run(() => InvokeAsync(context)).GetAwaiter().GetResult(); }

      public async Task<int> InvokeAsync(InvocationContext context) {
        string config = context.ParseResult.GetValueForOption(this.configOption);

        bool configOptionProvided = context.ParseResult.FindResultFor(this.configOption) != null;

        if (configOptionProvided && string.IsNullOrEmpty(config)) {
            ConsoleCalls.PrintError("Error: --config option requires a valid file path.");
            throw new ArgumentException("The --config option requires a valid file path.");
        }

        if (!string.IsNullOrEmpty(config)) {
            Environment.SetEnvironmentVariable("NYRA_BOT_CONFIG_PATH", config);
            ConsoleCalls.PrintStatus($"Using config: {config}");
        }
        else {
            ConsoleCalls.PrintStatus("No config specified; using defaults.");
        }

        ConsoleCalls.PrintStatus("Creating instance");
        ConsoleCalls.PrintStatus("Testing network connectivity to Discord");

        try {
          using var ping = new Ping();
          string host = "discord.com";
          byte[] buffer = new byte[32];
          int timeout = 1000;
          var reply = ping.Send(host, timeout, buffer, new PingOptions());

          if (reply.Status == IPStatus.Success) {
              ConsoleCalls.PrintStatus("Networking test has passed");
          }
          else {
              ConsoleCalls.PrintWarning($"Ping returned status {reply.Status}");
          }
        }
        catch (Exception ex) {
          ConsoleCalls.PrintWarning($"Cannot contact Discord: {ex.Message}");
        }

        await Bot.Nyra.RunAsync();
        return 0;
      }
    }
  }
}
