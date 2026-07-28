DROP INDEX IF EXISTS idx__users__username;
ALTER TABLE users DROP COLUMN IF EXISTS username;
