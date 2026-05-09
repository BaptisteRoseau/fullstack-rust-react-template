CREATE TABLE IF NOT EXISTS api_key (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    hash VARCHAR(256) UNIQUE NOT NULL,
    name VARCHAR(256) NOT NULL,
    owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permissions JSON NOT NULL DEFAULT '[]',
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx__api_key__owner ON api_key (owner);
CREATE INDEX IF NOT EXISTS idx__api_key__hash ON api_key (hash);
