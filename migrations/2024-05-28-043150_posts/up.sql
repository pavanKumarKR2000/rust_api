-- Your SQL goes here

CREATE TABLE posts(
    id SERIAL,
    name TEXT NOT NULL,
    details JSONB NOT NULL,
    PRIMARY KEY(id,name)
)