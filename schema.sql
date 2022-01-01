CREATE TABLE schema_version (
    major INT4 -- a table or column name is no longer valid
  , minor INT4 -- there's a new table or column
  , CONSTRAINT schema_version_pkey
);
INSERT INTO schema_version VALUES (0, 0);

CREATE TABLE languages (
    id INT4 PRIMARY KEY -- TODO: make xxhash(name)
  , name TEXT UNIQUE
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
  id INT4 PRIMARY KEY -- TODO: make xxhash(version, family)
  , family TEXT -- I've got aspirations to expand this suite to cover other SQLs
                -- and the SQL specs themselves
  , version TEXT UNIQUE
  , CONSTRAINT unique_version UNIQUE (family, version)
);
-- TODO: move to xxhash(name)
INSERT INTO versions VALUES
    (0, "postgres", "10")
  , (1, "postgres", "11")
  , (2, "postgres", "12")
  , (3, "postgres", "13")
  , (4, "postgres", "14");

CREATE TABLE language_versions (
  language_id INT4 REFERENCES languages(id)
  , version_id INT4 REFERENCES versions(id)
  , CONSTRAINT language_version_pkey PRIMARY KEY (language_id, version_id)
);

INSERT INTO language_versions
  SELECT l.id AS language_id, v.id AS version_id
  FROM versions AS v, languages AS l; -- implicit full outer join

CREATE TABLE statements ( -- TODO: maybe rename to "documents" or "inputs"
  id INT8 PRIMARY KEY      -- the xxhash of the text
  , "text" TEXT NOT NULL
  -- TODO: consider moving fingerprint to its own table
  , language_id INT4 REFERENCES languages(id) -- just a hint for the oracles.
    -- One snippet might might be valid in multiple languages (e.g. all pgsql is
    -- valid psql)
);

CREATE TABLE statement_fingerprints(
    fingerprint INT8
  , statement_id INT8 REFERENCES statements(id)
  , CONSTRAINT statement_fingerprints_pkey PRIMARY KEY (fingerprint, statement_id)
);

CREATE INDEX statement_fingerprints_idx ON statement_fingerprints(statement_id);

CREATE TABLE urls (
    id int8 PRIMARY KEY -- xxhash of the url itself
  , url TEXT UNIQUE
  , license_id INT8 REFERENCES licenses(id)
);

CREATE TABLE statement_sources (
    url_id INT8 REFERENCES urls
  , statement_id INT8 REFERENCES statements(id)
  , locator TEXT -- which helps locate the statement within the page defined by the url
  , CONSTRAINT statement_source_pkey PRIMARY KEY (url_id, statement_id, locator)
);
CREATE INDEX statements_for_source ON statement_sources(url_id, statement_id);

CREATE TABLE licenses (
    id TEXT PRIMARY KEY -- The short-form identifier for the license. Where
                        -- possible, it should be an identifier from https://spdx.org/licenses/
  , "text" TEXT NOT NULL -- the full text of the license
);

CREATE TABLE oracles(
  id INT8 -- xxhash3_64 of the oracle name, which should include the language
          -- name and version name
  , name TEXT
  , language_id INT8 REFERENCES languages(id)
  , version_id INT8 REFERENCES versions(id)
);

CREATE TABLE testimony(
  statement_id INT8 REFERENCES statements(id)
  , oracle_id INT8 REFERENCES oracles(id)
  , valid BOOLEAN
  , oration TEXT -- any extra output from the oracle, given for context
  , CONSTRAINT testimony_pkey PRIMARY KEY (statement_id, oracle_id)
);
-- TODO: maybe CREATE INDEX testimony_by_oracle ON testimonty(oracle_id); ?
