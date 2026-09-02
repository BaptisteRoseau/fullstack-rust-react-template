CREATE TABLE IF NOT EXISTS directories (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES directories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx__directories__owner ON directories (owner);
CREATE INDEX IF NOT EXISTS idx__directories__parent_id ON directories (parent_id);
