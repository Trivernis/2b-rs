-- Your SQL goes here
CREATE TABLE youtube_songs (
    id BIGSERIAL PRIMARY KEY,
    spotify_id VARCHAR(255) NOT NULL,
    artist VARCHAR(128) NOT NULL,
    title VARCHAR(255) NOT NULL,
    album VARCHAR(255) NOT NULL,
    url VARCHAR(128) NOT NULL,
    score INTEGER DEFAULT 0 NOT NULL,
    UNIQUE (spotify_id, url)
)