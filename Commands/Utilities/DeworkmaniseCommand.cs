/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Utilities/DeworkmaniseCommand.cs
    Authors: Invra
    Notes: A tool to parse garbled workman
*/

namespace Nyra.Commands {
  [Category("Utility")]
  public class DeworkmaniseCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public DeworkmaniseCommand(DiscordSocketClient client) {
      this.client = client;
    }

    private static readonly Dictionary<char, char> translationBooklet = new Dictionary<char, char> {
      ['q'] = 'q',
      ['w'] = 'd',
      ['e'] = 'r',
      ['r'] = 'w',
      ['t'] = 'b',
      ['y'] = 'j',
      ['u'] = 'f',
      ['i'] = 'u',
      ['o'] = 'p',
      ['p'] = ';',
      ['['] = '[',
      [']'] = ']',
      ['\\'] = '\\',
      ['a'] = 'a',
      ['s'] = 's',
      ['d'] = 'h',
      ['f'] = 't',
      ['g'] = 'g',
      ['h'] = 'y',
      ['j'] = 'n',
      ['k'] = 'e',
      ['l'] = 'o',
      [';'] = 'i',
      ['\''] = '\'',
      ['z'] = 'z',
      ['x'] = 'x',
      ['c'] = 'm',
      ['v'] = 'c',
      ['b'] = 'v',
      ['n'] = 'k',
      ['m'] = 'l',
      [','] = ',',
      ['.'] = '.',
      ['/'] = '/'
    };

    private static readonly Dictionary<char, char> ShiftPairs = new Dictionary<char, char> {
      ['1'] = '!',
      ['2'] = '@',
      ['3'] = '#',
      ['4'] = '$',
      ['5'] = '%',
      ['6'] = '^',
      ['7'] = '&',
      ['8'] = '*',
      ['9'] = '(',
      ['0'] = ')',
      ['-'] = '_',
      ['='] = '+',
      ['['] = '{',
      [']'] = '}',
      ['\\'] = '|',
      [';'] = ':',
      ['\''] = '"',
      [','] = '<',
      ['.'] = '>',
      ['/'] = '?',
      ['`'] = '~'
    };

    private static readonly Dictionary<char, char> ShiftedToUnshifted = ShiftPairs.ToDictionary(kvp => kvp.Value, kvp => kvp.Key);

    [Command("deworkmanise")]
    [Alias(["hrdpwelakusr"])]
    [Summary("Decodes what somone has typed which was meant to be on the Workman Keylayout, but typed on QWERTY.")]
    public async Task DeworkmaniseAsync([Remainder] string input = "") {
      string textToDecode = input;

      if (string.IsNullOrWhiteSpace(input) && Context.Message.Reference != null) {
        var refMsg = await Context.Channel.GetMessageAsync(Context.Message.Reference.MessageId.Value);
        if (refMsg != null && refMsg is IUserMessage userMsg) {
          textToDecode = userMsg.Content;
          string decoded = Decode(textToDecode);

          await userMsg.ReplyAsync(decoded);

          try {
            await Context.Message.DeleteAsync();
          } catch {
          }
          return;
        }
      }

      if (!string.IsNullOrWhiteSpace(textToDecode)) {
        string decoded = Decode(textToDecode);
        await ReplyAsync(decoded);
      } else {
        await ReplyAsync("No text provided and no message replied to.");
      }
    }

    private string Decode(string input) {
      var sb = new StringBuilder();

      foreach (char ch in input) {
        bool wasShifted = false;
        char baseChar = ch;

        if (char.IsLetter(ch)) {
          baseChar = char.ToLowerInvariant(ch);
          wasShifted = char.IsUpper(ch);
        } else if (ShiftedToUnshifted.TryGetValue(ch, out var unshift)) {
          baseChar = unshift;
          wasShifted = true;
        } else {
          baseChar = ch;
          wasShifted = false;
        }

        if (translationBooklet.TryGetValue(baseChar, out var mappedChar)) {
          char outChar = mappedChar;

          if (wasShifted) {
            if (char.IsLetter(mappedChar)) {
              outChar = char.ToUpperInvariant(mappedChar);
            } else if (ShiftPairs.TryGetValue(mappedChar, out var shiftedMapped)) {
              outChar = shiftedMapped;
            }
          }

          sb.Append(outChar);
        } else {
          sb.Append(ch);
        }
      }

      return sb.ToString();
    }
  }
}
