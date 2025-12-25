-- This trigger enforces the following lines to every table created in the database:
--
-- -- Every table has those fields
-- created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
-- updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
-- -- Every table has the following trigger to automatically set/update those fields
-- CREATE OR REPLACE TRIGGER update_<table_name>__updated_at BEFORE
-- UPDATE ON items FOR EACH ROW EXECUTE FUNCTION update_modified_column();
--
CREATE OR REPLACE FUNCTION add_item_creation_and_update_fields_trigger() RETURNS event_trigger LANGUAGE plpgsql AS $$
DECLARE cmd RECORD;
tbl text;
clean text;
idx_name text;
trg_name text;
BEGIN FOR cmd IN
SELECT *
FROM pg_event_trigger_ddl_commands() LOOP IF cmd.object_type = 'table' THEN tbl := cmd.object_identity;
-- Add the columns in the table definition
EXECUTE format(
    'ALTER TABLE %s ADD COLUMN IF NOT EXISTS created_at WITH TIME ZONE DEFAULT now() NOT NULL',
    tbl
);
EXECUTE format(
    'ALTER TABLE %s ADD COLUMN IF NOT EXISTS updated_at WITH TIME ZONE DEFAULT now() NOT NULL',
    tbl
);
-- Create the trigger to automatically update "updated_at" field upon modification
IF NOT EXISTS (
    SELECT 1
    FROM pg_trigger t
        JOIN pg_class c ON t.tgrelid = c.oid
        JOIN pg_namespace n ON c.relnamespace = n.oid
    WHERE t.tgname = trg_name
        AND (n.nspname || '.' || c.relname) = replace(clean, '__', '.')
) THEN EXECUTE format(
    'CREATE TRIGGER %I BEFORE UPDATE ON %s FOR EACH ROW EXECUTE FUNCTION update_modified_column()',
    trg_name,
    tbl
);
END IF;
END IF;
END LOOP;
END;
$$;
-- Install the event trigger for every CREATE TABLE statements
DROP EVENT TRIGGER IF EXISTS ensure_timestamps_ddl;
CREATE EVENT TRIGGER ensure_timestamps_ddl ON ddl_command_end
WHEN TAG IN ('CREATE TABLE') EXECUTE PROCEDURE add_item_creation_and_update_fields_trigger();