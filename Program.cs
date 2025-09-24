namespace TerryDavis {
  class Program {
    static async Task Main(string[] args) {
      // Print non-intrusive warning. As running this as admin
      // in the future is a really dumb idea.
      if (Environment.IsPrivilegedProcess) {
        Console.ForegroundColor = ConsoleColor.Yellow;
        Console.WriteLine("WARNING: This process is running with elevated privileges. Running as an elevated user is not recommended and may pose security risks.");
        Console.ResetColor();
      }

      DotNetEnv.Env.Load();
      var bot = new Bot();
      await bot.RunAsync();
    }
  }
}
