CREATE TABLE guild_settings (
    guild_id BIGINT NOT NULL,
    key VARCHAR(255) NOT NULL,
    value VARCHAR(1024),
    PRIMARY KEY (guild_id, key)
);