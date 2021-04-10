<h1 align="center">
2B (Tobi) Discord bot.
</h1>
<p align="center">
A rust rewrite of the originally js 2B bot.
</p>

## Current feature set

- minecraft information
- playing music from youtube
- miscellaneous commands

## System Dependencies

The bot depends on a few programs to be installed on the system.

### Data Storage

- [postgresql](https://www.postgresql.org/)


### Music

- [FFmpeg](https://github.com/FFmpeg/FFmpeg)
- [youtube-dl](https://github.com/ytdl-org/youtube-dl)


### Misc Commands

- [qalculate](https://github.com/Qalculate/libqalculate)


## API Dependencies

The bot depends on the following APIs

- [Discord](https://discord.com/developers/applications): It's a discord bot...
- [Spotify](https://developer.spotify.com/documentation/web-api/): To fetch song names to be searched on youtube for music playback
- [lyrics.ohv](https://lyricsovh.docs.apiary.io): To fetch lyrics for playing songs
- [SauceNAO](https://saucenao.com): To fetch source information for images


## Dev Dependencies

- Rust
- Other stuff that you have to figure out yourself because it just works for me


## Configuration

The bot reads all required configuration values from the environment or optionally from a .env file.
The required values are:
- `BOT_TOKEN` (required): Discord bot token
- `BOT_OWNER` (required): Discord UserID of the bot owner
- `DATABASE_URL` (required): Connection uri to the postgres database in the schema `postgres://myuser:mypassword@localhost:5432/database`
- `SPOTIFY_CLIENT_ID` (required): Spotify API Client ID
- `SPOTIFY_CLIENT_SECRET` (required): Spotify API Client Secret
- `SAUCENAO_API_KEY` (required): SauceNAO API Key
- `BOT_PREFIX` (optional): The prefix of the bot. Defaults to `~` if not set.
- `LOG_DIR` (optional): Directory to store log files in. Defaults to `logs` in the cwd.


## License

It's GPL 3.0