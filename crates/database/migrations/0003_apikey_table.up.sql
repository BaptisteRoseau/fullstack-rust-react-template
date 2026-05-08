CREATE TABLE IF NOT EXISTS apikey (
    key varchar(256) UNIQUE NOT NULL,
    owner FOREIGN KEY NOT NULL,
    permissions JSON NOT NULL DEFAULT '[]'
    PRIMARY KEY(key)
);
