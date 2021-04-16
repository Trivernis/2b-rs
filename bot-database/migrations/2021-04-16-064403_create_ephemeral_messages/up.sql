-- Your SQL goes here
CREATE TABLE ephemeral_messages (
    channel_id BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    timeout TIMESTAMP NOT NULL,
    PRIMARY KEY (channel_id, message_id)
)