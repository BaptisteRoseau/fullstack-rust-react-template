CREATE TABLE IF NOT EXISTS directory_permissions (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    directory_id UUID NOT NULL REFERENCES directories(id) ON DELETE CASCADE,
    grantee UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_level VARCHAR(16) NOT NULL CHECK (permission_level IN ('viewer','editor','manager')),
    granted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY(id),
    CONSTRAINT uq__directory_permissions UNIQUE (directory_id, grantee)
);

CREATE INDEX IF NOT EXISTS idx__directory_permissions__directory_id ON directory_permissions (directory_id);
CREATE INDEX IF NOT EXISTS idx__directory_permissions__grantee ON directory_permissions (grantee);
