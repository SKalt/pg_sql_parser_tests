INSERT INTO predictions (
    statement_id
  , oracle_id
  , language_id
  , "message"
  , "error"
  , valid
) VALUES (
    ? -- 1: statement_id
  , ? -- 2: oracle_id
  , ? -- 3: language_id
  , ? -- 4: message
  , ? -- 5: error
  , ? -- 6: whether the statement is explicitly valid/not
) ON CONFLICT DO NOTHING;