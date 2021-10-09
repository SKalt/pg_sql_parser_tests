SET enable_partition_pruning = on;
EXPLAIN SELECT count(*) FROM measurement WHERE logdate >= DATE '2008-01-01';
--                                     QUERY PLAN
-- -------------------------------------------------------------------&zwsp;----------------
--  Aggregate  (cost=37.75..37.76 rows=1 width=8)
--    ->  Seq Scan on measurement_y2008m01  (cost=0.00..33.12 rows=617 width=0)
--          Filter: (logdate >= '2008-01-01'::date)
