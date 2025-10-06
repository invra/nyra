using System.Runtime.InteropServices;
using System.Runtime.Intrinsics.X86;

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

    [LibraryImport("libhardwareinfo", EntryPoint = "get_mem_heap_usize")]
    public static partial ulong GetTotalMemoryHeap();
  }

  public class GetHardwareInfo {
    private string cpuModel;
    private int cpuCores;
    private ulong memoryTotal;
    private string osVersion;

    public GetHardwareInfo(
      string cpuModel = null,
      int cpuCores = 8,
      ulong memoryTotal = 8192,
      string osVersion = "macOS"
    ) {
      this.cpuModel = Ffi.GetCpuModelSafe();
      this.cpuCores = Ffi.GetCpuCoreCount();
      this.memoryTotal = Ffi.GetTotalMemoryHeap();
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

    public ulong MemoryTotal {
      get => memoryTotal;
      set => memoryTotal = value;
    }

    public string OsVersion {
      get => osVersion;
      set => osVersion = value;
    }
  }
}
