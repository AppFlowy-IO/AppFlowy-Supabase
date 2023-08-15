-- Create the user_awareness partition of the af_collab_update table
CREATE TABLE af_collab_update_user_awareness PARTITION OF af_collab_update FOR
VALUES IN (5);
-- Add the encrypt column to the af_collab_update table. If the encrypt column is
-- null, then the update is not encrypted.
ALTER TABLE af_collab_update
ADD COLUMN encrypt INTEGER DEFAULT NULL;

-- Add encryption_sign column to the af_user table
ALTER TABLE af_user
ADD COLUMN encryption_sign TEXT;
CREATE OR REPLACE FUNCTION prevent_reset_encryption_sign_func() RETURNS TRIGGER AS $$ BEGIN IF OLD.encryption_sign IS NOT NULL
    AND NEW.encryption_sign IS DISTINCT
FROM OLD.encryption_sign THEN RAISE EXCEPTION 'encryption_sign can not be reset once it has been set';
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
-- Currently, we aren't using the JWT token for requests, so there's no need to validate
-- auth.jwt() ->> 'email' against the email.
ALTER POLICY af_user_update_policy ON public.af_user USING (true) WITH CHECK (true);