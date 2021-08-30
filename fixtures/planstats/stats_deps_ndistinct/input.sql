DROP STATISTICS stts;
CREATE STATISTICS stts (dependencies, ndistinct) ON a, b FROM t;
ANALYZE t;
EXPLAIN (ANALYZE, TIMING OFF) SELECT COUNT(*) FROM t GROUP BY a, b;
--                                        QUERY PLAN                                        
-- -------------------------------------------------------------------&zwsp;-------------------------
--  HashAggregate  (cost=220.00..221.00 rows=100 width=16) (actual rows=100 loops=1)
--    Group Key: a, b
--    ->  Seq Scan on t  (cost=0.00..145.00 rows=10000 width=8) (actual rows=10000 loops=1)
