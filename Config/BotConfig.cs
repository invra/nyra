using System;

namespace TerryDavis.Config
{
    public class BotConfig
    {
        public string Prefix { get; }
        public string Token { get; }

        public BotConfig()
        {
            Prefix = Environment.GetEnvironmentVariable("BOT_PREFIX") ?? "!";
            Token = Environment.GetEnvironmentVariable("BOT_TOKEN") ?? "";
        }
    }
}
