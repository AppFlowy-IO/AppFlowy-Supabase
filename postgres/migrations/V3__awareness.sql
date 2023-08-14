CREATE TABLE af_collab_update_user_awareness PARTITION OF af_collab_update FOR
VALUES IN (5);

-- Add the encrypt column to the af_collab_update table. If the encrypt column is
-- null, then the update is not encrypted.
ALTER TABLE af_collab_update ADD COLUMN encrypt INTEGER DEFAULT null;
