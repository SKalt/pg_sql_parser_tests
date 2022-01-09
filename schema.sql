CREATE TABLE schema_version (
    major INT4 -- a table or column name is no longer valid
  , minor INT4 -- there's a new table or column
  , CONSTRAINT schema_version_pkey
);
INSERT INTO schema_version VALUES (0, 0);

CREATE TABLE languages (
    id INT4 PRIMARY KEY -- TODO: make xxhash(name)
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
  id INT4 PRIMARY KEY -- xxhash64(family, version)
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
  id INT8 PRIMARY KEY      -- the xxhash of the text
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
    fingerprint INT8
  , statement_id INT8 REFERENCES statements(id)
  , CONSTRAINT statement_fingerprints_pkey PRIMARY KEY (statement_id, fingerprint)
);

CREATE TABLE statement_versions(
    statement_id INT8 REFERENCES statements(id)
  , version_id INT8 REFERENCES versions(id)
  , CONSTRAINT statement_versions_pkey PRIMARY KEY (statement_id, version_id)
);
CREATE UNIQUE INDEX version_statements_idx ON statement_versions(version_id, statement_id);

CREATE TABLE urls (
    id int8 PRIMARY KEY -- xxhash of the url itself
  , "url" TEXT UNIQUE
  , license_id INT8 REFERENCES licenses(id)
);

CREATE TABLE statement_sources (
    statement_id INT8 REFERENCES statements(id)
  , url_id INT8 REFERENCES urls
  , start_line INT
  , start_offset INT
  , end_line INT
  , end_offset INT
  , locator TEXT -- which helps locate the statement within the page defined by the url
  , CONSTRAINT statement_source_pkey PRIMARY KEY (url_id, statement_id, start_offset)
);
CREATE INDEX statements_for_source ON statement_sources(statement_id, url_id, start_offset);

CREATE TABLE licenses (
    id TEXT PRIMARY KEY -- The short-form identifier for the license. Where
                        -- possible, it should be an identifier from https://spdx.org/licenses/
  , "text" TEXT NOT NULL -- the full text of the license
);

CREATE TABLE oracles(
   id INT8 -- xxhash3_64 of the oracle name
  , "name" TEXT -- e.g. "postgres 13 no-op do-block".
);

CREATE TABLE predictions(
    statement_id INT8 REFERENCES statements(id)
  , oracle_id INT8 REFERENCES oracles(id) -- encodes version
  , language_id INT8 REFERENCES languages(id)
  , error TEXT -- bonus debugging text if there was an error; doesn't mean the
               -- statement isn't valid
  , "message" TEXT -- any extra output from the oracle, hopefully something like
                   -- json {syntax/parse tree, tokens}. This column is just
                   -- for debugging, so don't sweat it and probably don't try to
                   -- parse it unless you're confident of its structure.
  , valid BOOLEAN
  , CONSTRAINT testimony_pkey PRIMARY KEY (statement_id, oracle_id)
);
