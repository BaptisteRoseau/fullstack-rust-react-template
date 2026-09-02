CREATE TABLE IF NOT EXISTS file_permissions (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    file_id UUID NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    grantee UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_level VARCHAR(16) NOT NULL CHECK (permission_level IN ('viewer','editor','manager')),
    granted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY(id),
    CONSTRAINT uq__file_permissions UNIQUE (file_id, grantee)
);

CREATE INDEX IF NOT EXISTS idx__file_permissions__file_id ON file_permissions (file_id);
CREATE INDEX IF NOT EXISTS idx__file_permissions__grantee ON file_permissions (grantee);
