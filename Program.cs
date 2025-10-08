/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: Program.cs
  Authors: Invra
  Notes: Entrypoint file
*/

using System.CommandLine;
using System.CommandLine.Invocation;
using System.Net.NetworkInformation;
using System.Runtime.InteropServices;
using Nyra.Colourise;
using Nyra.Gui;
using Nyra.Stdout;

namespace Nyra {
  public static class BotLauncher {
    [UnmanagedCallersOnly(EntryPoint = "start_bot")]
    public static void StartBot(IntPtr configPtr) {
      string config = configPtr == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(configPtr);
      Task.Run(async () => {
        if (!string.IsNullOrEmpty(config)) {
          Environment.SetEnvironmentVariable("NYRA_BOT_CONFIG_PATH", config);
          ConsoleCalls.PrintStatus($"Using config: {config}");
        } else {
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
          } else {
            ConsoleCalls.PrintWarning($"Ping returned status {reply.Status}");
          }
        } catch (Exception ex) {
          ConsoleCalls.PrintWarning($"Cannot contact Discord: {ex.Message}");
        }

        await Bot.Nyra.RunAsync();
      }).GetAwaiter().GetResult();
    }

    public static async Task<int> Start(string config) {
      if (!string.IsNullOrEmpty(config)) {
        Environment.SetEnvironmentVariable("NYRA_BOT_CONFIG_PATH", config);
        ConsoleCalls.PrintStatus($"Using config: {config}");
      } else {
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
        } else {
          ConsoleCalls.PrintWarning($"Ping returned status {reply.Status}");
        }
      } catch (Exception ex) {
        ConsoleCalls.PrintWarning($"Cannot contact Discord: {ex.Message}");
      }

      await Bot.Nyra.RunAsync();
      return 0;
    }
  }

  class Program {
    static int Main(string[] args) {
      var configOption = new Option<string>(
          aliases: new[] { "--config", "-c" },
          description: "Path to the configuration file") {
        IsRequired = false
      };

      var guiOption = new Option<bool>(
          aliases: new[] { "--gui" },
          description: "Launch the GUI instead of the bot") {
        IsRequired = false
      };

      var rootCommand = new RootCommand("Nyra Discord bot");
      rootCommand.Add(configOption);
      rootCommand.Add(guiOption);
      rootCommand.Handler = new CommandHandlerImpl(configOption, guiOption);

      return rootCommand.Invoke(args);
    }

    private class CommandHandlerImpl : ICommandHandler {
      private readonly Option<string> configOption;
      private readonly Option<bool> guiOption;

      public CommandHandlerImpl(Option<string> configOption, Option<bool> guiOption) {
        this.configOption = configOption;
        this.guiOption = guiOption;
      }

      public int Invoke(InvocationContext context) {
        bool launchGui = context.ParseResult.GetValueForOption(this.guiOption);

        if (launchGui) {
          ConsoleCalls.PrintStatus("Launching GUI...");
          NyraGui.Start();
          return 0;
        }

        string config = context.ParseResult.GetValueForOption(this.configOption);
        bool configOptionProvided = context.ParseResult.FindResultFor(this.configOption) != null;

        if (configOptionProvided && string.IsNullOrEmpty(config)) {
          ConsoleCalls.PrintError("Error: --config option requires a valid file path.");
          throw new ArgumentException("The --config option requires a valid file path.");
        }

        return Task.Run(() => BotLauncher.Start(config)).GetAwaiter().GetResult();
      }

      public async Task<int> InvokeAsync(InvocationContext context) {
        string config = context.ParseResult.GetValueForOption(this.configOption);
        return await BotLauncher.Start(config);
      }
    }
  }
}
