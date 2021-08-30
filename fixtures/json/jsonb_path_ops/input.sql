CREATE INDEX idxginp ON api USING GIN (jdoc jsonb_path_ops);
