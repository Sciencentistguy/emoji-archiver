# emoji-archiver

## Deprecated
The functionality of this bot has been merged into [discord-channel-archiver](https://github.com/Sciencentistguy/discord-channel-archiver),
and therefore this bot is deprecated and unmaintained.

---

A small discord bot to archive the emojis from a server

## Usage

- Edit `src/main.rs` and change the value of the constant `PATH_PREFIX` to your desired download location.
- [Create](https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account) a discord application and bot.
- [Invite](https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot) the bot to your server.
- Run the bot with `cargo run`. To provide the token, you have 3 options:
  - Provide the token directly with `--token <token>`
  - Provide the name of a file containing the token with `--token-filename <filename>`
  - Set the environment variable `DISCORD_TOKEN` to the token before running.
- Send the message `!archive`
- Sit back and watch the bot download all the emojis from the server into the directory you specified.

---

Based on [Serenity](https://github.com/serenity-rs/serenity).

Available under the terms of the GNU AGPL.
