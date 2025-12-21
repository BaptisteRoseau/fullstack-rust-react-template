-- Run this to audit all user tables for created_at/updated_at and the updated_at trigger
-- Makes sure every table has those fields correctly set-up
SELECT n.nspname AS schema,
    c.relname AS table,
    EXISTS (
        SELECT 1
        FROM information_schema.columns col
        WHERE col.table_schema = n.nspname
            AND col.table_name = c.relname
            AND col.column_name = 'created_at'
    ) AS has_created_at,
    EXISTS (
        SELECT 1
        FROM information_schema.columns col
        WHERE col.table_schema = n.nspname
            AND col.table_name = c.relname
            AND col.column_name = 'updated_at'
    ) AS has_updated_at,
    EXISTS (
        SELECT 1
        FROM pg_trigger t
        WHERE t.tgrelid = c.oid
            AND t.tgname LIKE 'update_%__updated_at'
    ) AS has_update_trigger
FROM pg_class c
    JOIN pg_namespace n ON c.relnamespace = n.oid
WHERE c.relkind = 'r' -- regular tables
    AND n.nspname NOT IN ('pg_catalog', 'information_schema')
ORDER BY n.nspname,
    c.relname;
