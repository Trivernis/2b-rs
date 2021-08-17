-- Your SQL goes here
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    name VARCHAR(128) NOT NULL,
    description VARCHAR(4096) NOT NULL,
    event_start timestamp NOT NULL,
    event_end timestamp
)