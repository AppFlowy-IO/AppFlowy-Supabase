CREATE TABLE af_collab_update_user_awareness PARTITION OF af_collab_update FOR
VALUES IN (5);

ALTER TABLE af_collab_update ADD COLUMN is_encrypt BOOLEAN DEFAULT false;
