using System.Diagnostics;
using System.Management;
using System.Runtime.InteropServices;
using Hardware.Info;

namespace Nyra.Commands {
  public class HostCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public HostCommand(DiscordSocketClient client) {
      this.client = client;
    }

    public static int GetPhysicalCores() {
      try {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows)) {
          return Environment.ProcessorCount;
        } else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux)) {
          int procCount = 0;
          foreach (var line in File.ReadAllText("/proc/cpuinfo").Split("\n")) {
            if (line.StartsWith("cpu cores")) {
              procCount = int.Parse(line.Split(":")[1].Trim());
              break;
            }
          }
          return procCount;
        } else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX)) {
          using var proc = new Process {
            StartInfo = new ProcessStartInfo {
              FileName = "/usr/sbin/sysctl",
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
        string prettyName = string.Empty;
        foreach (var line in File.ReadAllText("/etc/os-release").Split("\n")) {
          if (line.StartsWith("PRETTY_NAME")) {
            prettyName = line.Split("=")[1].Replace("\"", "");
          }
        }
        return prettyName;
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

        return $"macOS {prettyName}";
      }

      if (OperatingSystem.IsWindows()) {
        using var searcher = new ManagementObjectSearcher("SELECT Caption FROM Win32_OperatingSystem");
        using var results = searcher.Get();

        foreach (ManagementObject os in results) {
          if (os["Caption"] is string caption && !string.IsNullOrWhiteSpace(caption)) {
            var match = Regex.Match(caption, @"Windows\s+(?:[A-Za-z]+)?(\s*\d+(\.\d*)?)?", RegexOptions.IgnoreCase);
            if (match.Success) return match.Value;
          }
        }
        return "Windows";
      }
      if (OperatingSystem.IsFreeBSD()) {
        return "FreeBSD";
      }
      return "Unknown";
    }

    private static (ulong totalRam, ulong usedRam) GetMemoryInfoMacOS() {
      try {
        using var proc = new Process {
          StartInfo = new ProcessStartInfo {
            FileName = "/usr/bin/vm_stat",
            RedirectStandardOutput = true,
            UseShellExecute = false
          }
        };
        proc.Start();
        string output = proc.StandardOutput.ReadToEnd();
        proc.WaitForExit();

        var lines = output.Split('\n');
        ulong pagesFree = 0, pagesActive = 0, pagesInactive = 0, pagesWired = 0, pagesCompressed = 0;
        foreach (var line in lines) {
          if (line.StartsWith("Pages free:")) {
            pagesFree = ulong.Parse(Regex.Match(line, @"\d+").Value);
          } else if (line.StartsWith("Pages active:")) {
            pagesActive = ulong.Parse(Regex.Match(line, @"\d+").Value);
          } else if (line.StartsWith("Pages inactive:")) {
            pagesInactive = ulong.Parse(Regex.Match(line, @"\d+").Value);
          } else if (line.StartsWith("Pages wired down:")) {
            pagesWired = ulong.Parse(Regex.Match(line, @"\d+").Value);
          } else if (line.StartsWith("Pages occupied by compressor:")) {
            pagesCompressed = ulong.Parse(Regex.Match(line, @"\d+").Value);
          }
        }

        using var sysctlProc = new Process {
          StartInfo = new ProcessStartInfo {
            FileName = "/usr/sbin/sysctl",
            Arguments = "-n hw.memsize",
            RedirectStandardOutput = true,
            UseShellExecute = false
          }
        };

        sysctlProc.Start();
        string memSizeOutput = sysctlProc.StandardOutput.ReadLine() ?? "0";
        sysctlProc.WaitForExit();
        ulong totalMemoryBytes = ulong.Parse(memSizeOutput);
        const ulong pageSize = 4096;
        ulong usedMemoryPages = pagesActive + pagesWired + pagesCompressed;
        ulong usedMemoryBytes = usedMemoryPages * pageSize;
        ulong totalMemoryMB = totalMemoryBytes / (1024 * 1024);
        ulong usedMemoryMB = usedMemoryBytes / (1024 * 1024);
        return (totalMemoryMB, usedMemoryMB);
      } catch {
        return (0, 0);
      }
    }

    [Command("host")]
    [Summary("Replies with system information.")]
    public async Task HostAsync() {
      var msg = await ReplyAsync("Getting system information...");

      var hardwareInfo = new HardwareInfo();
      hardwareInfo.RefreshAll();

      var cpu = hardwareInfo.CpuList.FirstOrDefault();
      string cpuName = cpu?.Name ?? "Unknown";

      string totalRamStr, usedRamStr;
      if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX)) {
        var (totalRam, usedRam) = GetMemoryInfoMacOS();
        totalRamStr = totalRam > 0 ? $"{totalRam} MB" : "Unknown";
        usedRamStr = totalRam > 0 ? $"{usedRam} MB" : "Unknown";
      } else {
        ulong totalRam = hardwareInfo.MemoryStatus.TotalPhysical;
        ulong availableRam = hardwareInfo.MemoryStatus.AvailablePhysical;
        ulong usedRam = totalRam - availableRam;
        totalRamStr = totalRam > 0 ? $"{totalRam / (1024 * 1024)} MB" : "Unknown";
        usedRamStr = totalRam > 0 ? $"{usedRam / (1024 * 1024)} MB" : "Unknown";
      }

      var embed = new EmbedBuilder()
          .WithTitle("Host System Information")
          .WithColor(Color.Purple)
          .AddField("CPU", cpuName, true)
          .AddField("Processors", $"{GetPhysicalCores()}", true)
          .AddField("RAM", $"{usedRamStr} / {totalRamStr}", true)
          .AddField("OS", OsName(), true)
          .AddField("64-bit Process", Environment.Is64BitProcess, true)
          .AddField("Elevated Proc", Environment.IsPrivilegedProcess ? "True :warning:" : "False", true)
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
