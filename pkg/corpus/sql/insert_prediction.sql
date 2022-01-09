INSERT INTO predictions (
    statement_id
  , oracle_id
  , language_id
  , version_id
  , "message"
  , "error"
  , valid
) VALUES (
    ? -- 1: statement_id
  , ? -- 2: oracle_id
  , ? -- 3: language_id
  , ? -- 4: version_id
  , ? -- 5: message
  , ? -- 6: error
  , ? -- 7: whether the statement is explicitly valid/not
);