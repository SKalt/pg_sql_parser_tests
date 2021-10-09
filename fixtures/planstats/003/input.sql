SELECT histogram_bounds FROM pg_stats
WHERE tablename='tenk1' AND attname='unique1';

--                    histogram_bounds
-- ------------------------------------------------------
--  {0,993,1997,3050,4040,5036,5957,7057,8029,9016,9995}
