using System.Runtime.InteropServices;

namespace Nyra.HardwareInfo {
  public static partial class Ffi {
      [LibraryImport("libhardwareinfo", EntryPoint = "get_cpu_model")]
      public static partial nint GetCpuModelPtr();

      [LibraryImport("libhardwareinfo", EntryPoint = "free_string")]
      public static partial void FreeString(nint ptr);

      public static string GetCpuModelSafe() {
          nint ptr = GetCpuModelPtr();
          if (ptr == IntPtr.Zero)
              return string.Empty;

          string result = Marshal.PtrToStringAnsi(ptr)!;
          FreeString(ptr);
          return result;
      }

      [LibraryImport("libhardwareinfo", EntryPoint = "get_cpu_core_count")]
      public static partial int GetCpuCoreCount();
  }
  public class GetHardwareInfo {
    private string cpuModel;
    private int cpuCores;
    private double ramSizeGB;
    private string osVersion;

    public GetHardwareInfo(
        string cpuModel = null,
        int cpuCores = 8,
        double ramSizeGB = 16.0,
        string osVersion = "macOS")
    {
        this.cpuModel = cpuModel ?? Ffi.GetCpuModelSafe();
        this.cpuCores = Ffi.GetCpuCoreCount();
        this.ramSizeGB = ramSizeGB;
        this.osVersion = osVersion;
    }

    public string CpuModel {
      get => cpuModel;
      set => cpuModel = value;
    }

    public int CpuCores {
      get => cpuCores;
      set => cpuCores = value;
    }

    public double RamSizeGB {
      get => ramSizeGB;
      set => ramSizeGB = value;
    }

    public string OsVersion {
      get => osVersion;
      set => osVersion = value;
    }
  }
}
