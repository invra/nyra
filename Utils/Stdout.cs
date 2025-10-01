/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Colourise.cs
    Authors: Invra
    Notes: Functions for different common printing messages
*/

using Nyra.Colourise;

namespace Nyra.Stdout {
  public static class ConsoleCalls {
    public static void PrintStatus(string message) =>
      Console.WriteLine("[STDOUT/status]:".Cyan().Bold() + " " + message);

    public static void PrintWarning(string message) =>
      Console.WriteLine("[STDOUT/warning]:".Yellow().Bold() + " " + message);

    public static void PrintError(string message) =>
      Console.Error.WriteLine("[STDERR/critical]:".Red().Bold() + " " + message);

    public static void PrintCustom(string message, string type) =>
      Console.WriteLine($"[STDOUT/{type}]:".Yellow().Bold() + " " + message);
  }
}
