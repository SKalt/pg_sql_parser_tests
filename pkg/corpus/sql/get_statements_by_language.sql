SELECT
    stmt.id
  , stmt.text
FROM languages AS lang
JOIN statement_languages AS stmt_lang
  ON lang.name = ?
  AND stmt_lang.language_id = lang.id
JOIN statements AS stmt ON stmt_lang.statement_id = stmt.id;
