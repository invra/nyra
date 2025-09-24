using Discord;
using Discord.Commands;
using Discord.WebSocket;

namespace TerryDavis.Commands
{
    public class BanCommand : ModuleBase<SocketCommandContext>
    {
        private readonly DiscordSocketClient _client;

        public BanCommand(DiscordSocketClient client)
        {
            _client = client;
        }

        [Command("ban")]
        [Summary("Bans a user by mention or ID.")]
        public async Task BanAsync(
            string userInput,
            [Remainder] string reason = "No reason provided"
        )
        {
            if (Context.Guild == null)
            {
                await ReplyAsync("This command cannot be used in DMs.");
                return;
            }

            var guildUser = ParseUser(userInput);
            if (guildUser == null)
            {
                await ReplyAsync("User not found. Make sure to mention them or use their ID.");
                return;
            }

            var mod = (SocketGuildUser)Context.User;
            if (!mod.GuildPermissions.BanMembers)
            {
                await ReplyAsync("You don't have permission to ban members!");
                return;
            }

            if (guildUser.Id == Context.User.Id)
            {
                await ReplyAsync("You can't ban yourself!");
                return;
            }

            if (guildUser.GuildPermissions.Administrator)
            {
                await ReplyAsync("You can't ban an administrator!");
                return;
            }

            try
            {
                await Context.Guild.AddBanAsync(guildUser, 0, reason);

                var msg = await ReplyAsync($"Banning {guildUser.Username}...");

                var embed = new EmbedBuilder()
                    .WithTitle("User Banned")
                    .WithColor(Color.Green)
                    .AddField(
                        "Banned User",
                        $"{guildUser.Username}#{guildUser.Discriminator}",
                        true
                    )
                    .AddField("Moderator", Context.User.Username, true)
                    .AddField("Reason", reason)
                    .WithCurrentTimestamp()
                    .Build();

                await msg.ModifyAsync(m =>
                {
                    m.Content = string.Empty;
                    m.Embed = embed;
                });
            }
            catch (Exception ex)
            {
                await ReplyAsync($"Failed to ban {guildUser.Username}: {ex.Message}");
            }
        }

        private SocketGuildUser? ParseUser(string input)
        {
            if (input.StartsWith("<@") && input.EndsWith(">"))
                input = input.Replace("<@!", "").Replace("<@", "").Replace(">", "");

            if (!ulong.TryParse(input, out ulong id))
                return null;

            return Context.Guild.GetUser(id);
        }
    }
}
