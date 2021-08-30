CREATE INDEX hidx ON testhstore USING GIST (h gist_hstore_ops(siglen=32));
