SELECT
    stmt.id
  , stmt.text
FROM statement_languages AS stmt_lang
JOIN statements AS stmt
  ON stmt_lang.language_id = ?
  AND stmt_lang.statement_id = stmt.id
LEFT JOIN predictions AS prediction
  ON prediction.statement_id = stmt.id
  AND prediction.oracle_id = ?
WHERE
  prediction.statement_id IS NULL
  OR prediction.oracle_id != ?;
