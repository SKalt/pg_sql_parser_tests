-- contrast all predictions on a statement
SELECT
  stmt.*,
  prediction.*
FROM statements AS stmt
JOIN predictions AS prediction ON prediction.statement_id = statement.id
JOIN oracles AS oracle ON
JOIN languages AS language ON
JOIN