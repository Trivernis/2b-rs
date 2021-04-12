-- Your SQL goes here
CREATE TABLE gifs (
    id BIGSERIAL PRIMARY KEY ,
    category VARCHAR(128),
    name VARCHAR(128),
    url VARCHAR(128) NOT NULL,
    UNIQUE (category, name)
)