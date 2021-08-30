CREATE INDEX path_gist_idx ON test USING GIST (path gist_ltree_ops(siglen=100));
