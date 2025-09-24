using System.Diagnostics;
using System.Management;
using System.Runtime.InteropServices;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using Hardware.Info;

namespace TerryDavis.Commands {
  public class HostCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient _client;

    public HostCommand(DiscordSocketClient client) {
      _client = client;
    }

    public static int GetPhysicalCores() {
      try {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows)) {
            return Environment.ProcessorCount;
        } else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux)) {
          var cpuInfo = System.IO.File.ReadAllText("/proc/cpuinfo");
          return cpuInfo.Split(new[] { "physical id" }, StringSplitOptions.None).Distinct().Count();
        } else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX)) {
          using var proc = new Process {
            StartInfo = new ProcessStartInfo {
              FileName = "sysctl",
              Arguments = "-n hw.physicalcpu",
              RedirectStandardOutput = true,
              UseShellExecute = false
            }
          };
          proc.Start();
          string? output = proc.StandardOutput.ReadLine();
          proc.WaitForExit();
          if (int.TryParse(output, out int cores))
            return cores;
        }
      } catch { }
      return Environment.ProcessorCount;
    }

    public static string OsName() {
      if (OperatingSystem.IsLinux()) {
        return "Linux (Distro detection TBI)";
      }

      if (OperatingSystem.IsMacOS()) {
        string version = "Unknown";
        try {
          using var proc = new Process {
            StartInfo = new ProcessStartInfo {
              FileName = "/usr/bin/sw_vers",
              Arguments = "-productVersion",
              RedirectStandardOutput = true,
              UseShellExecute = false
            }
          };
          proc.Start();
          version = proc.StandardOutput.ReadLine() ?? "Unknown";
        } catch { }

        int major = 0;
        if (int.TryParse(version.Split('.')[0], out var parsed))
          major = parsed;

        string prettyName = major switch {
          10 when version.StartsWith("10.13") => "High Sierra",
          10 when version.StartsWith("10.14") => "Mojave",
          10 when version.StartsWith("10.15") => "Catalina",
          11 => "Big Sur",
          12 => "Monterey",
          13 => "Ventura",
          14 => "Sonoma",
          15 => "Sequoia",
          26 => "Tahoe",
          _ => "Unknown"
        };

        return $"macOS {prettyName} ({version})";
      }

      if (OperatingSystem.IsWindows()) {
          string result = string.Empty;
          ManagementObjectSearcher searcher = new ManagementObjectSearcher("SELECT Caption FROM Win32_OperatingSystem");
          foreach (ManagementObject os in searcher.Get())
          {
              result = os["Caption"].ToString();
              break;
          }
          return result;
      }

      if (OperatingSystem.IsFreeBSD()) {
        return "FreeBSD";
      }

      return "Unknown";
    }

    [Command("host")]
    [Summary("Replies with system information.")]
    public async Task HostAsync() {
      var msg = await ReplyAsync("Getting system information...");

      var hardwareInfo = new HardwareInfo();
      hardwareInfo.RefreshAll();

      var cpu = hardwareInfo.CpuList.FirstOrDefault();
      string cpuName = cpu?.Name ?? "Unknown";

      ulong totalRam = hardwareInfo.MemoryStatus.TotalPhysical;
      ulong availableRam = hardwareInfo.MemoryStatus.AvailablePhysical;
      ulong usedRam = totalRam - availableRam;

      string totalRamStr = totalRam > 0 ? $"{totalRam / (1024 * 1024)} MB" : "Unknown";
      string usedRamStr = totalRam > 0 ? $"{usedRam / (1024 * 1024)} MB" : "Unknown";

      var embed = new EmbedBuilder()
          .WithTitle("Host System Information")
          .WithColor(Color.Purple)
          .AddField("CPU", cpuName, true)
          .AddField("Processors (P Cores)", $"{GetPhysicalCores()}", true)
          .AddField("RAM", $"{usedRamStr} / {totalRamStr}", true)
          .AddField("OS", OsName(), true)
          .AddField("64-bit Process", Environment.Is64BitProcess, true)
          .WithFooter(f => f.Text = $"Requested by {Context.User.Username}")
          .WithCurrentTimestamp()
          .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }
  }
}
