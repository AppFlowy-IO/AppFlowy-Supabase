-- Create the user_awareness partition of the af_collab_update table
CREATE TABLE af_collab_update_user_awareness PARTITION OF af_collab_update FOR
VALUES IN (5);
-- Currently, we aren't using the JWT token for requests, so there's no need to validate
-- auth.jwt() ->> 'email' against the email.
ALTER POLICY af_user_update_policy ON public.af_user USING (true) WITH CHECK (true);