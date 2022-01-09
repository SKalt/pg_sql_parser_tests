SELECT
    stmt.id
  , stmt.text
FROM languages
JOIN statement_languages stmt_lang ON lang.id = ? AND lang.id = stmt_lang.language_id
JOIN statements stmt ON stmt.id = stmt_lang.stmt_id;
