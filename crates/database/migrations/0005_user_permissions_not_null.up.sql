UPDATE users SET permissions = '[]' WHERE permissions IS NULL;
ALTER TABLE users ALTER COLUMN permissions SET DEFAULT '[]';
ALTER TABLE users ALTER COLUMN permissions SET NOT NULL;
