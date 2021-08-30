EXPLAIN (ANALYZE, TIMING OFF) SELECT COUNT(*) FROM t GROUP BY a;
--                                        QUERY PLAN                                        
-- -------------------------------------------------------------------&zwsp;----------------------
--  HashAggregate  (cost=195.00..196.00 rows=100 width=12) (actual rows=100 loops=1)
--    Group Key: a
--    ->  Seq Scan on t  (cost=0.00..145.00 rows=10000 width=4) (actual rows=10000 loops=1)
