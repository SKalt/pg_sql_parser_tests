select * from t1 where ctid = '(0, 1)';
-- ERROR:  could not access status of transaction 4007513275
-- DETAIL:  Could not open file "pg_xact/0EED": No such file or directory.

select heap_force_kill('t1'::regclass, ARRAY['(0, 1)']::tid[]);
--  heap_force_kill 
-- -----------------
 
-- (1 row)

select * from t1 where ctid = '(0, 1)';
-- (0 rows)
