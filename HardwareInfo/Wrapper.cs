using System;

namespace Nyra.HardwareInfo {
  public class GetHardwareInfo {
    private string cpuModel;
    private int cpuCores;
    private double ramSizeGB;
    private string osVersion;

    public GetHardwareInfo(
        string cpuModel = "Intel i7-12700",
        int cpuCores = 8,
        double ramSizeGB = 16.0,
        string osVersion = "Windows 11") {
      this.cpuModel = cpuModel;
      this.cpuCores = cpuCores;
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
