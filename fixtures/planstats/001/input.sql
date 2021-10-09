SELECT relpages, reltuples FROM pg_class WHERE relname = 'tenk1';

--  relpages | reltuples
-- ----------+-----------
--       358 |     10000
