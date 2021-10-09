CREATE INDEX btreeidx ON tbloom (i1, i2, i3, i4, i5, i6);
-- CREATE INDEX
SELECT pg_size_pretty(pg_relation_size('btreeidx'));
--  pg_size_pretty
-- ----------------
--  3976 kB
-- (1 row)
EXPLAIN ANALYZE SELECT * FROM tbloom WHERE i2 = 898732 AND i5 = 123451;
--                                               QUERY PLAN                                              
-- -------------------------------------------------------------------&zwsp;-----------------------------------
--  Seq Scan on tbloom  (cost=0.00..2137.00 rows=2 width=24) (actual time=12.805..12.805 rows=0 loops=1)
--    Filter: ((i2 = 898732) AND (i5 = 123451))
--    Rows Removed by Filter: 100000
--  Planning Time: 0.138 ms
--  Execution Time: 12.817 ms
-- (5 rows)
