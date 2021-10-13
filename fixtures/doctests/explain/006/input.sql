EXPLAIN SELECT sum(i) FROM foo WHERE i < 10;

                             QUERY PLAN
-------------------------------------------------------------------&zwsp;--
 Aggregate  (cost=23.93..23.93 rows=1 width=4)
   ->  Index Scan using fi on foo  (cost=0.00..23.92 rows=6 width=4)
         Index Cond: (i < 10)
(3 rows)
