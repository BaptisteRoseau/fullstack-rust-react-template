CREATE DATABASE mydatabase;
\c mydatabase;
CREATE EXTENSION "uuid-ossp";
-- Created and Updated helpers
-- Requires the table to have an `updated_at` field
-- Use as follows:
-- CREATE OR REPLACE TRIGGER update_<YOUR_TABLE>_updated_at
--     BEFORE UPDATE ON <YOUR_TABLE>
--     FOR EACH ROW EXECUTE FUNCTION update_modified_column();
-- CREATE INDEX index_<YOUR_TABLE>__created_at ON <YOUR_TABLE>(created_at);
CREATE OR REPLACE FUNCTION update_modified_column() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = now();
RETURN NEW;
END;
$$ language 'plpgsql';
-- -----------------------------------------------------------------------------
-- ITEMS
-- -----------------------------------------------------------------------------
CREATE TABLE items (
    id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    PRIMARY KEY(id),
    CONSTRAINT fk_user_id FOREIGN KEY(id) REFERENCES items(id) ON DELETE
    SET DEFAULT
);
CREATE INDEX index_items__name ON items(name);
CREATE INDEX index_items__owner_id ON items(owner_id);
CREATE INDEX index_items__created_at ON items(created_at);
CREATE OR REPLACE TRIGGER update_items__updated_at BEFORE
UPDATE ON items FOR EACH ROW EXECUTE FUNCTION update_modified_column();
