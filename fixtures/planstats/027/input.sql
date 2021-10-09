EXPLAIN (ANALYZE, TIMING OFF) SELECT * FROM t WHERE a = 1 AND b = 1;
--                                  QUERY PLAN                                  
-- -------------------------------------------------------------------&zwsp;----------
--  Seq Scan on t  (cost=0.00..195.00 rows=1 width=8) (actual rows=100 loops=1)
--    Filter: ((a = 1) AND (b = 1))
--    Rows Removed by Filter: 9900
