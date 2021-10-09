CREATE INDEX bloomidx ON tbloom USING bloom (i1, i2, i3, i4, i5, i6);
-- CREATE INDEX
SELECT pg_size_pretty(pg_relation_size('bloomidx'));
--  pg_size_pretty
-- ----------------
--  1584 kB
-- (1 row)

EXPLAIN ANALYZE SELECT * FROM tbloom WHERE i2 = 898732 AND i5 = 123451;
--                                                      QUERY PLAN                                                      
-- -------------------------------------------------------------------&zwsp;--------------------------------------------------
--  Bitmap Heap Scan on tbloom  (cost=1792.00..1799.69 rows=2 width=24) (actual time=0.388..0.388 rows=0 loops=1)
--    Recheck Cond: ((i2 = 898732) AND (i5 = 123451))
--    Rows Removed by Index Recheck: 29
--    Heap Blocks: exact=28
--    ->  Bitmap Index Scan on bloomidx  (cost=0.00..1792.00 rows=2 width=0) (actual time=0.356..0.356 rows=29 loops=1)
--          Index Cond: ((i2 = 898732) AND (i5 = 123451))
--  Planning Time: 0.099 ms
--  Execution Time: 0.408 ms
-- (8 rows)
