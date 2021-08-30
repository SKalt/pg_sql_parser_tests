SELECT collation for (description) FROM pg_description LIMIT 1;
--  pg_collation_for
-- ------------------
--  "default"

SELECT collation for ('foo' COLLATE "de_DE");
--  pg_collation_for
-- ------------------
--  "de_DE"
