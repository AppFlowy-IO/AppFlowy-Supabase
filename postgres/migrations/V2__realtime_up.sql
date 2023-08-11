CREATE ROLE anon;
CREATE ROLE authenticated;

-- Add the did(device_id) column to the af_collab_update table
ALTER TABLE af_collab_update ADD COLUMN did TEXT DEFAULT '';
-- Enable RLS on the af_collab_update table
ALTER TABLE af_collab_update ENABLE ROW LEVEL SECURITY;
CREATE POLICY af_collab_update_policy
ON af_collab_update
FOR ALL
TO anon, authenticated
USING (true);

-- Enable RLS on the af_user table
-- Policy for INSERT
ALTER TABLE af_user ENABLE ROW LEVEL SECURITY;
CREATE POLICY af_user_insert_policy
ON public.af_user
FOR INSERT
TO anon, authenticated
WITH CHECK (true);
-- Policy for UPDATE
CREATE POLICY af_user_update_policy
ON public.af_user
FOR UPDATE
USING (auth.jwt() ->> 'email' = email)
WITH CHECK (auth.jwt() ->> 'email' = email);
-- Policy for SELECT
CREATE POLICY af_user_select_policy
ON public.af_user
FOR SELECT
TO anon, authenticated
USING (true);

ALTER TABLE af_user FORCE ROW LEVEL SECURITY;

-- Enable RLS on the af_workspace_member table
ALTER TABLE af_workspace_member ENABLE ROW LEVEL SECURITY;
CREATE POLICY af_workspace_member_policy
ON af_workspace_member
FOR ALL
TO anon, authenticated
USING (true);
ALTER TABLE af_workspace_member FORCE ROW LEVEL SECURITY;

-- Enable RLS on the af_workspace table
ALTER TABLE af_workspace ENABLE ROW LEVEL SECURITY;
CREATE POLICY af_workspace_policy
ON af_workspace
FOR ALL
TO anon, authenticated
USING (true);
ALTER TABLE af_workspace FORCE ROW LEVEL SECURITY;

-- Update the flush_collab_updates function that accept a new column called did
CREATE OR REPLACE FUNCTION public.flush_collab_updates_v2(
      oid TEXT,
      new_value BYTEA,
      md5 TEXT,
      value_size INTEGER,
      partition_key INTEGER,
      uid BIGINT,
      workspace_id UUID,
      removed_keys BIGINT [],
      did TEXT
   ) RETURNS void AS $$
DECLARE lock_key INTEGER;
BEGIN -- Hashing the oid to an integer for the advisory lock
lock_key := (hashtext(oid)::bigint)::integer;
-- Getting a session level lock
PERFORM pg_advisory_lock(lock_key);
-- Deleting rows with keys in removed_keys
DELETE FROM af_collab_update
WHERE key = ANY (removed_keys);
-- Inserting a new update with the new key and value
INSERT INTO af_collab_update(
    oid,
    value,
    md5,
    value_size,
    partition_key,
    uid,
    workspace_id,
    did
)
VALUES (
    oid,
    new_value,
    md5,
    value_size,
    partition_key,
    uid,
    workspace_id,
    did
);
-- Releasing the lock
PERFORM pg_advisory_unlock(lock_key);
RETURN;
END;
$$ LANGUAGE plpgsql;
