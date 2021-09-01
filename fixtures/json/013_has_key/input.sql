CREATE INDEX idxgintags ON api USING GIN ((jdoc -> 'tags'));
