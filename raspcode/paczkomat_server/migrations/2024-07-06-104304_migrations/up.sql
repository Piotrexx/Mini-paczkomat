CREATE TABLE lockers (
        lockerid VARCHAR(50) PRIMARY KEY NOT NULL,
        gpio INT NOT NULL,
        is_empty BOOLEAN NOT NULL DEFAULT 1 
);