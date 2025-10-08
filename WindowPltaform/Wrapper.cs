/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    WindowPlatform/Wrapper.cs
    Authors: Invra
    Notes: Library interop and constructor file
*/

using System.Runtime.InteropServices;

namespace Nyra.Gui {
  public static partial class NyraGui {
    [DllImport("libnyra_gui", EntryPoint = "init_gui", CallingConvention = CallingConvention.Cdecl)]
    private static extern void InitGui(IntPtr config, IntPtr start_bot);

    public static unsafe void Start(string? config = null) {
      IntPtr configPtr = IntPtr.Zero;
      if (!string.IsNullOrEmpty(config)) {
        configPtr = Marshal.StringToCoTaskMemUTF8(config);
      }

      IntPtr startBotPtr = (IntPtr)(delegate* unmanaged<nint, void>)&Nyra.BotLauncher.StartBot;

      try {
        Console.WriteLine($"Calling InitGui with configPtr: {configPtr}, startBotPtr: {startBotPtr}");
        InitGui(configPtr, startBotPtr);
      } finally {
        if (configPtr != IntPtr.Zero)
          Marshal.FreeCoTaskMem(configPtr);
      }
    }
  }
}
