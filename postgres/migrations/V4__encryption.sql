-- Add the encrypt column to the af_collab_update table. If the encrypt column is
-- 0, then the update is not encrypted.
ALTER TABLE af_collab_update
ADD COLUMN encrypt INTEGER DEFAULT 0;
-- Add the encrypt column to the af_collab_snapshot table. If the encrypt column is
-- 0, then the update is not encrypted.
ALTER TABLE af_collab_snapshot
ADD COLUMN encrypt INTEGER DEFAULT 0;
-- Add encryption_sign column to the af_user table
ALTER TABLE af_user
ADD COLUMN encryption_sign TEXT;
CREATE OR REPLACE FUNCTION prevent_reset_encryption_sign_func() RETURNS TRIGGER AS $$ BEGIN IF OLD.encryption_sign IS NOT NULL
    AND NEW.encryption_sign IS DISTINCT
FROM OLD.encryption_sign THEN RAISE EXCEPTION 'The encryption sign can not be reset once it has been set';
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_prevent_reset_encryption_sign BEFORE
UPDATE ON af_user FOR EACH ROW EXECUTE FUNCTION prevent_reset_encryption_sign_func();

-- Re-create the af_user_profile_view to show the 'encrypt' column in the view.
DROP VIEW af_user_profile_view;
CREATE VIEW af_user_profile_view AS
SELECT u.*,
    w.workspace_id AS latest_workspace_id
FROM af_user u
    INNER JOIN (
        SELECT uid,
            workspace_id,
            rank() OVER (
                PARTITION BY uid
                ORDER BY updated_at DESC
            ) AS rn
        FROM af_workspace_member
    ) w ON u.uid = w.uid
    AND w.rn = 1;
-- Update the flush_collab_updates function that accept a new column called encrypt
CREATE OR REPLACE FUNCTION public.flush_collab_updates_v3(
        oid TEXT,
        new_value BYTEA,
        encrypt INTEGER,
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
        encrypt,
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
        encrypt,
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