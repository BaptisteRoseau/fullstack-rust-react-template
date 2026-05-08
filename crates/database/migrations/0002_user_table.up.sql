CREATE TABLE IF NOT EXISTS users (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    last_name varchar(128) NOT NULL,
    first_name varchar(128) NOT NULL,
    email varchar(255) NOT NULL,
    permissions JSON DEFAULT '[]'
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx__users__last_name ON users (last_name);
CREATE INDEX IF NOT EXISTS idx__users__first_name ON users (first_name);
CREATE INDEX IF NOT EXISTS idx__users__email ON users (email);
