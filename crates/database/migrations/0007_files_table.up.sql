CREATE TABLE IF NOT EXISTS files (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES directories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    storage_key VARCHAR(512) UNIQUE NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    size_bytes BIGINT NOT NULL,
    stored_size_bytes BIGINT NOT NULL,
    is_compressed BOOLEAN NOT NULL DEFAULT false,
    encrypted_dek BYTEA NOT NULL,
    dek_nonce BYTEA NOT NULL,
    content_nonce BYTEA NOT NULL,
    thumbnail_storage_key VARCHAR(512),
    thumbnail_nonce BYTEA,
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx__files__owner ON files (owner);
CREATE INDEX IF NOT EXISTS idx__files__parent_id ON files (parent_id);
