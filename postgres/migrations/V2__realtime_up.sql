-- Add the did(device_id) column to the af_collab_update table
ALTER TABLE af_collab_update ADD COLUMN did TEXT DEFAULT '';
-- Enable RLS on the af_collab_update table
ALTER TABLE af_collab_update ENABLE ROW LEVEL SECURITY;
-- Insert policy of af_collab_update table
CREATE POLICY af_collab_update_insert_policy
ON af_collab_update
FOR INSERT
TO anon, authenticated
USING (true);
ALTER POLICY af_collab_update_insert_policy ON af_collab_update USING (true);
-- Delete policy of af_collab_update table
CREATE POLICY af_collab_update_delete_policy
ON af_collab_update
FOR DELETE
TO anon
USING (true);
ALTER POLICY af_collab_update_delete_policy ON af_collab_update USING (true);
-- Select policy of af_collab_update table
CREATE POLICY af_collab_update_select_policy
ON af_collab_update
FOR SELECT
TO anon, authenticated
USING (true);
ALTER POLICY af_collab_update_select_policy ON af_collab_update USING (true);
