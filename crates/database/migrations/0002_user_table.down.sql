-- Add down migration script here
DROP TABLE IF EXISTS users;
DROP INDEX IF EXISTS idx__users__last_name;
DROP INDEX IF EXISTS idx__users__first_name;
DROP INDEX IF EXISTS idx__users__email;