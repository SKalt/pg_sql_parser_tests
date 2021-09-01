CREATE INDEX trgm_idx ON test_trgm USING GIN (t gin_trgm_ops);
