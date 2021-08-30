vacuum t1;
-- ERROR:  found xmin 507 from before relfrozenxid 515
-- CONTEXT:  while scanning block 0 of relation "public.t1"

select ctid from t1 where xmin = 507;
--  ctid  
-- -------
--  (0,3)
-- (1 row)

select heap_force_freeze('t1'::regclass, ARRAY['(0, 3)']::tid[]);
--  heap_force_freeze 
-- -------------------
 
-- (1 row)

select ctid from t1 where xmin = 2;
--  ctid
-- -------
--  (0,3)
-- (1 row)
