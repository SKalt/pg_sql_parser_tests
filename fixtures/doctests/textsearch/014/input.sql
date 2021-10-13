CREATE INDEX textsearch_idx ON pgweb USING GIN (textsearchable_index_col);
