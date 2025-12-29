CREATE TABLE IF NOT EXISTS users (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    last_name varchar(128) NOT NULL,
    first_name varchar(128) NOT NULL,
    email varchar(255) NOT NULL,
    address varchar(255) DEFAULT NULL,
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx_users_last_name ON users (last_name);
CREATE INDEX IF NOT EXISTS idx_users_first_name ON users (first_name);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);