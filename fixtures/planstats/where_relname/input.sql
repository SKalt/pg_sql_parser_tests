SELECT relpages, reltuples FROM pg_class WHERE relname = 't';

--  relpages | reltuples
-- ----------+-----------
--        45 |     10000
