/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Utilities/ColourCommand.cs
    Authors: Invra
    Notes: A colour tool command
*/

using System.Globalization;
using System.Text.Json;
using SkiaSharp;

namespace Nyra.Commands {
  [Category("Utility")]
  public class ColourCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;
    private readonly HttpClient httpClient = new HttpClient();

    public ColourCommand(DiscordSocketClient client) {
      this.client = client;
    }

    [Command("colour")]
    [Alias("color")]
    [Summary("Shows detailed info about a color.")]
    public async Task ColourAsync([Remainder] string input) {
      SKColor? color = ParseColor(input);
      if (color == null) {
        await ReplyAsync("Could not parse that color. Try `#RRGGBB` or `rgb(r,g,b)`.");
        return;
      }

      var c = color.Value;
      var hsvValues = RGBtoHSV(c);

      string rgb = $"RGB({c.Red},{c.Green},{c.Blue})";
      string bgr = $"BGR({c.Blue},{c.Green},{c.Red})";
      string hex = $"#{c.Red:X2}{c.Green:X2}{c.Blue:X2}";
      string hsv = $"HSV({hsvValues.H:0},{hsvValues.S * 100:0}%,{hsvValues.V * 100:0}%)";
      string cmyk = RGBtoCMYK(c.Red, c.Green, c.Blue);
      string colorName = await GetColorNameAsync(hex);
      string pantone = await GetPantoneApproximationAsync(hex);

      string tempFile = Path.Combine(Path.GetTempPath(), $"color-{c.Red}-{c.Green}-{c.Blue}.png");
      using (var bmp = new SKBitmap(128, 128))
      using (var canvas = new SKCanvas(bmp)) {
        canvas.Clear(new SKColor(c.Red, c.Green, c.Blue));
        using var image = SKImage.FromBitmap(bmp);
        using var data = image.Encode(SKEncodedImageFormat.Png, 100);
        using var stream = File.OpenWrite(tempFile);
        data.SaveTo(stream);
      }

      var embed = new EmbedBuilder()
          .WithTitle($"Color info: {input}")
          .WithColor(new Discord.Color(c.Red, c.Green, c.Blue))
          .AddField("HEX", hex, true)
          .AddField("RGB", rgb, true)
          .AddField("BGR", bgr, true)
          .AddField("HSV", hsv, true)
          .AddField("CMYK", cmyk, true)
          .AddField("Name", colorName, true)
          .AddField("Pantone", pantone, true)
          .WithImageUrl("attachment://color.png")
          .WithFooter(f => f.Text = $"Requested by {Context.User.Username}")
          .WithCurrentTimestamp()
          .Build();

      await Context.Channel.SendFileAsync(tempFile, embed: embed);
      File.Delete(tempFile);
    }

    private SKColor? ParseColor(string input) {
      input = input.Trim().ToLower();

      if (input.StartsWith("#")) {
        if (input.Length == 4)
          input = $"#{input[1]}{input[1]}{input[2]}{input[2]}{input[3]}{input[3]}";

        if (uint.TryParse(input.Substring(1), NumberStyles.HexNumber, null, out uint hexVal)) {
          byte r = (byte)((hexVal >> 16) & 255);
          byte g = (byte)((hexVal >> 8) & 255);
          byte b = (byte)(hexVal & 255);
          return new SKColor(r, g, b);
        }
      } else if (input.StartsWith("rgb")) {
        var parts = input.Substring(input.IndexOf("(") + 1).TrimEnd(')').Split(',');
        if (parts.Length == 3 &&
            byte.TryParse(parts[0], out byte r) &&
            byte.TryParse(parts[1], out byte g) &&
            byte.TryParse(parts[2], out byte b)) {
          return new SKColor(r, g, b);
        }
      }

      return null;
    }

    private string RGBtoCMYK(int r, int g, int b) {
      double rd = r / 255.0, gd = g / 255.0, bd = b / 255.0;
      double k = 1 - Math.Max(rd, Math.Max(gd, bd));
      if (k >= 1) return "0,0,0,100";
      double c = (1 - rd - k) / (1 - k);
      double m = (1 - gd - k) / (1 - k);
      double y = (1 - bd - k) / (1 - k);
      return $"{c * 100:0}%,{m * 100:0}%,{y * 100:0}%,{k * 100:0}%";
    }

    private async Task<string> GetColorNameAsync(string hex) {
      try {
        var response = await httpClient.GetStringAsync($"https://www.thecolorapi.com/id?hex={hex.Substring(1)}");
        using var doc = JsonDocument.Parse(response);
        return doc.RootElement.GetProperty("name").GetProperty("value").GetString() ?? "Unknown";
      } catch {
        return "Unknown";
      }
    }

    private async Task<string> GetPantoneApproximationAsync(string hex) {
      try {
        var response = await httpClient.GetStringAsync($"https://www.thecolorapi.com/id?hex={hex.Substring(1)}");
        using var doc = JsonDocument.Parse(response);
        var cmyk = doc.RootElement.GetProperty("cmyk");
        int c = cmyk.GetProperty("c").GetInt32();
        int m = cmyk.GetProperty("m").GetInt32();
        int y = cmyk.GetProperty("y").GetInt32();
        int k = cmyk.GetProperty("k").GetInt32();
        return $"C:{c} M:{m} Y:{y} K:{k}";
      } catch {
        return "N/A";
      }
    }

    private (double H, double S, double V) RGBtoHSV(SKColor c) {
      double r = c.Red / 255.0;
      double g = c.Green / 255.0;
      double b = c.Blue / 255.0;
      double max = Math.Max(r, Math.Max(g, b));
      double min = Math.Min(r, Math.Min(g, b));
      double delta = max - min;
      double h = 0;
      if (delta != 0) {
        if (max == r) h = 60 * (((g - b) / delta) % 6);
        else if (max == g) h = 60 * (((b - r) / delta) + 2);
        else if (max == b) h = 60 * (((r - g) / delta) + 4);
      }
      if (h < 0) h += 360;
      double s = max == 0 ? 0 : delta / max;
      double v = max;
      return (h, s, v);
    }
  }
}
