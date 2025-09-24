namespace TerryDavis {
  class Program {
    static async Task Main(string[] args) {
      DotNetEnv.Env.Load();
      var bot = new Bot();
      await bot.RunAsync();
    }
  }
}
