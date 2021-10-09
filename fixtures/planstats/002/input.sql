EXPLAIN SELECT * FROM tenk1 WHERE unique1 < 1000;

--                                    QUERY PLAN
-- -------------------------------------------------------------------&zwsp;-------------
--  Bitmap Heap Scan on tenk1  (cost=24.06..394.64 rows=1007 width=244)
--    Recheck Cond: (unique1 < 1000)
--    ->  Bitmap Index Scan on tenk1_unique1  (cost=0.00..23.80 rows=1007 width=0)
--          Index Cond: (unique1 < 1000)
