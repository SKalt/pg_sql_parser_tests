EXPLAIN SELECT * FROM foo;

--                        QUERY PLAN
-- ---------------------------------------------------------
--  Seq Scan on foo  (cost=0.00..155.00 rows=10000 width=4)
-- (1 row)
