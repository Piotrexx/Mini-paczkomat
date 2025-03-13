-- Your SQL goes here
CREATE TABLE locker (
    id VARCHAR NOT NULL PRIMARY KEY,
    locker_gpio INTEGER NOT NULL,
    io_type BOOLEAN NOT NULL,
    is_empty BOOLEAN NOT NULL DEFAULT TRUE
)