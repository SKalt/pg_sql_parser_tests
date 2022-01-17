CREATE TABLE schema_version (
    major INT4 -- a table or column name is no longer valid
  , minor INT4 -- there's a new table or column
  , CONSTRAINT schema_version_pkey PRIMARY KEY (major, minor)
);
INSERT INTO schema_version VALUES (0, 0);

CREATE TABLE languages (
    id INTEGER PRIMARY KEY -- TODO: make xxhash(name)? Not worth it for now
  , "name" TEXT UNIQUE
  -- , CONSTRAINT version_url_id_fkey FOREIGN KEY (url_id) REFERENCES urls.id
);

INSERT INTO languages VALUES
    (-1, "other")
  , (0, "pgsql")
  , (1, "plpgsql")
  , (2, "psql")
  , (3, "plperl")
  , (4, "pltcl")
  , (5, "plpython2")
  , (6, "plpython3");

-- for coordinating compatibility:
CREATE TABLE versions(
  id INTEGER PRIMARY KEY -- xxhash64(family, version)
  , family TEXT -- I've got aspirations to expand this suite to cover other SQLs
  , "version" TEXT
  , CONSTRAINT unique_version UNIQUE (family, version)
);
CREATE INDEX versions_order ON versions(family, "version");


CREATE TABLE language_versions (
    language_id INT4 REFERENCES languages(id)
  , version_id INT4 REFERENCES versions(id)
  , CONSTRAINT language_version_pkey PRIMARY KEY (language_id, version_id)
);

CREATE TABLE statements (
  id INTEGER PRIMARY KEY      -- the xxhash of the text
  , "text" TEXT NOT NULL
);

CREATE TABLE statement_languages (
    statement_id INT8 REFERENCES statements(id)
  , language_id INT4 REFERENCES languages(id) -- just a hint for the oracles.
    -- One snippet might might be valid in multiple languages (e.g. all pgsql is
    -- valid psql)
  , CONSTRAINT statement_languages_pkey PRIMARY KEY (language_id, statement_id)
);

CREATE TABLE statement_fingerprints(
    fingerprint INTEGER
  , statement_id INTEGER REFERENCES statements(id)
  , CONSTRAINT statement_fingerprints_pkey PRIMARY KEY (statement_id, fingerprint)
);

-- TODO: delete?
CREATE TABLE statement_versions(
    statement_id INT8 REFERENCES statements(id)
  , version_id INT8 REFERENCES versions(id)
  , CONSTRAINT statement_versions_pkey PRIMARY KEY (statement_id, version_id)
);
CREATE UNIQUE INDEX version_statements_idx ON statement_versions(version_id, statement_id);

CREATE TABLE licenses (
    id TEXT PRIMARY KEY -- The short-form identifier for the license. Where
                        -- possible, it should be an identifier from https://spdx.org/licenses/
  , "text" TEXT NOT NULL -- the full text of the license
);

CREATE TABLE urls (
    id INTEGER PRIMARY KEY -- xxhash3_64 of the url itself
  , "url" TEXT UNIQUE
  , license_id INT8 REFERENCES licenses(id)
);

-- this is dumb; maybe eliminate in favor of indices
CREATE TABLE documents (
    id INTEGER PRIMARY KEY -- xxhash_64 of the document
);

CREATE TABLE document_urls(
    document_id REFERENCES documents(id)
  , url_id REFERENCES urls(id)
  , CONSTRAINT document_url_pkey PRIMARY KEY (document_id, url_id)
);
CREATE UNIQUE INDEX urls_for_document ON document_urls(url_id, document_id);

CREATE TABLE document_statements (
    document_id INTEGER REFERENCES documents(id)
  , statement_id INTEGER REFERENCES statements(id)
  , start_line INTEGER
  , start_offset INTEGER
  , end_line INTEGER
  , end_offset INTEGER
  , locator TEXT -- which helps locate the statement within the page defined by the url.
                 -- Usually null, since the line number and byte offset are usually enough
  , CONSTRAINT document_statement_source_pkey PRIMARY KEY (document_id, statement_id, start_offset)
);
CREATE INDEX statements_for_source ON document_statements(statement_id, document_id, start_offset);

CREATE TABLE oracles(
   id INTEGER PRIMARY KEY -- xxhash3_64 of the oracle name
  , "name" TEXT -- e.g. "postgres 13 no-op do-block".
);

CREATE TABLE predictions(
    statement_id INTEGER REFERENCES statements(id)
  , oracle_id INTEGER REFERENCES oracles(id) -- encodes version
  , language_id INTEGER REFERENCES languages(id)
  , error TEXT -- bonus debugging text if there was an error; doesn't mean the
               -- statement isn't valid
  , "message" TEXT -- any extra output from the oracle, hopefully something like
                   -- json {syntax/parse tree, tokens}. This column is just
                   -- for debugging, so don't sweat it and probably don't try to
                   -- parse it unless you're confident of its structure.
  , valid BOOLEAN
  , CONSTRAINT testimony_pkey PRIMARY KEY (statement_id, oracle_id)
);
CREATE INDEX predictions_by_oracle ON predictions(oracle_id, statement_id);
