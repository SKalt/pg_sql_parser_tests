CREATE INDEX words_idx ON words USING GIN (word gin_trgm_ops);
