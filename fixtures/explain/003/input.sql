EXPLAIN SELECT * FROM foo WHERE i = 4;

--                          QUERY PLAN
-- --------------------------------------------------------------
--  Index Scan using fi on foo  (cost=0.00..5.98 rows=1 width=4)
--    Index Cond: (i = 4)
-- (2 rows)
