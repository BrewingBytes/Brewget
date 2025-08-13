-- Add up migration script here
CREATE TABLE tokens (
    token VARCHAR(255),
    email VARCHAR(255),
    PRIMARY KEY(token, email)
)
