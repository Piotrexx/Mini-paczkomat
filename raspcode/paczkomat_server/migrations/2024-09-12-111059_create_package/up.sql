-- Your SQL goes here
CREATE TABLE package (
        packageid INT PRIMARY KEY NOT NULL,
        locker_id TEXT NOT NULL,
        FOREIGN KEY (locker_id) REFERENCES lockers(lockerid)
);