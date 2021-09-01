CREATE INDEX path_gist_idx ON test USING GIST (array_path gist__ltree_ops(siglen=100));
