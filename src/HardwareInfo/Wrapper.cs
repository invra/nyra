/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/Wrapper.cs
    Authors: Invra
    Notes: Library interop and constructor file
*/

using System.Runtime.InteropServices;
using System.Runtime.Intrinsics.X86;

namespace Nyra.HardwareInfo {
  public static partial class Ffi {
    [LibraryImport("hardwareinfo", EntryPoint = "get_cpu_model")]
    public static partial nint GetCpuModelPtr();

    [LibraryImport("hardwareinfo", EntryPoint = "free_string")]
    public static partial void FreeString(nint ptr);

    public static string GetCpuModelSafe() {
      nint ptr = GetCpuModelPtr();
      if (ptr == IntPtr.Zero)
        return string.Empty;

      string result = Marshal.PtrToStringAnsi(ptr)!;
      FreeString(ptr);
      return result;
    }

    [LibraryImport("hardwareinfo", EntryPoint = "get_cpu_core_count")]
    public static partial int GetCpuCoreCount();

    [LibraryImport("hardwareinfo", EntryPoint = "get_mem_heap_usize")]
    public static partial ulong GetTotalMemoryHeap();

    [LibraryImport("hardwareinfo", EntryPoint = "get_mem_used_usize")]
    public static partial ulong GetTotalMemoryUsed();

    [LibraryImport("hardwareinfo", EntryPoint = "get_host_os_string")]
    public static partial nint GetHostOperatingSystemPtr();

    public static string GetHostOperatingSystemSafe() {
      nint ptr = GetHostOperatingSystemPtr();
      if (ptr == IntPtr.Zero)
        return string.Empty;

      string result = Marshal.PtrToStringAnsi(ptr)!;
      FreeString(ptr);
      return result;
    }
  }

  public class GetHardwareInfo {
    private string cpuModel;
    private int cpuCores;
    private double memoryTotal;
    private double memoryUsed;
    private string osVersion;

    public GetHardwareInfo(
      string cpuModel = null,
      int cpuCores = 8,
      double memoryTotal = 64.0,
      string osVersion = "macOS"
    ) {
      this.cpuModel = Ffi.GetCpuModelSafe();
      this.cpuCores = Ffi.GetCpuCoreCount();
      this.memoryTotal = (Ffi.GetTotalMemoryHeap() / Math.Pow(1024, 3));
      this.memoryUsed = (Ffi.GetTotalMemoryUsed() / Math.Pow(1024, 3));
      this.osVersion = Ffi.GetHostOperatingSystemSafe();
    }

    public string CpuModel {
      get => cpuModel;
      set => cpuModel = value;
    }

    public int CpuCores {
      get => cpuCores;
      set => cpuCores = value;
    }

    public double MemoryTotal {
      get => memoryTotal;
      set => memoryTotal = value;
    }

    public double MemoryUsed {
      get => memoryUsed;
      set => memoryUsed = value;
    }

    public string OsVersion {
      get => osVersion;
      set => osVersion = value;
    }
  }
}
