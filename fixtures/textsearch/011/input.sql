CREATE INDEX pgweb_idx ON pgweb USING GIN (to_tsvector(config_name, body));
