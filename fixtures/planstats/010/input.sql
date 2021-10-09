EXPLAIN SELECT * FROM tenk1 WHERE stringu1 = 'xxx';

--                         QUERY PLAN
-- ----------------------------------------------------------
--  Seq Scan on tenk1  (cost=0.00..483.00 rows=15 width=244)
--    Filter: (stringu1 = 'xxx'::name)
