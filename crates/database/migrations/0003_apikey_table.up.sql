CREATE TABLE IF NOT EXISTS api_key (
    key varchar(256) UNIQUE NOT NULL,
    owner FOREIGN KEY NOT NULL,
    permissions JSON NOT NULL DEFAULT '[]'
    PRIMARY KEY(key)
);

CREATE INDEX IF NOT EXISTS idx__api_key__owner ON api_key (owner);
