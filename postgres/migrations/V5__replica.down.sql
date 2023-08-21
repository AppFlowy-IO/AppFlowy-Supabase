DO $$
BEGIN
   IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'af_workspace_member_pkey')
   THEN
ALTER TABLE af_workspace_member DROP CONSTRAINT af_workspace_member_pkey;
END IF;
END $$;
