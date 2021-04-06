CREATE TABLE IF NOT EXISTS guilds (
    guild_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS guild_settings (
     guild_id INTEGER NOT NULL,
     setting_key TEXT NOT NULL,
     setting_value TEXT,
     FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
);