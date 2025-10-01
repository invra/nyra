namespace Nyra.Colourise {
  public static class AnsiExtensions {
    private const string Reset = "\u001b[0m";

    public static string Red(this string text) => $"\u001b[31m{text}{Reset}";
    public static string Green(this string text) => $"\u001b[32m{text}{Reset}";
    public static string Yellow(this string text) => $"\u001b[33m{text}{Reset}";
    public static string Blue(this string text) => $"\u001b[34m{text}{Reset}";
    public static string Magenta(this string text) => $"\u001b[35m{text}{Reset}";
    public static string Cyan(this string text) => $"\u001b[36m{text}{Reset}";
    public static string White(this string text) => $"\u001b[37m{text}{Reset}";

    public static string Bold(this string text) => $"\u001b[1m{text}{Reset}";
    public static string Underline(this string text) => $"\u001b[4m{text}{Reset}";
    public static string Blink(this string text) => $"\u001b[5m{text}{Reset}";
    public static string RapidBlink(this string text) => $"\u001b[6m{text}{Reset}";
  }
}
