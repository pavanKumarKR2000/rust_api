-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL,
    details TEXT NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (id,name)
);
