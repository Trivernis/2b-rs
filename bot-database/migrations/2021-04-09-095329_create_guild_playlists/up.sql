-- Your SQL goes here
CREATE TABLE guild_playlists (
    guild_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(1024) NOT NULL,
    PRIMARY KEY (guild_id, name)
)