-- Your SQL goes here
CREATE TABLE statistics (
    id BIGSERIAL PRIMARY KEY,
    version VARCHAR(32) NOT NULL,
    command VARCHAR(255) NOT NULL,
    executed_at TIMESTAMP NOT NULL,
    success BOOLEAN NOT NULL DEFAULT TRUE,
    error_msg TEXT
)