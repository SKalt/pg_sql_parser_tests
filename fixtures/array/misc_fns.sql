SELECT array_prepend(1, ARRAY[2,3]);
--  array_prepend
-- ---------------
--  {1,2,3}
-- (1 row)

SELECT array_append(ARRAY[1,2], 3);
--  array_append
-- --------------
--  {1,2,3}
-- (1 row)

SELECT array_cat(ARRAY[1,2], ARRAY[3,4]);
--  array_cat
-- -----------
--  {1,2,3,4}
-- (1 row)

SELECT array_cat(ARRAY[[1,2],[3,4]], ARRAY[5,6]);
--       array_cat
-- ---------------------
--  {{1,2},{3,4},{5,6}}
-- (1 row)

SELECT array_cat(ARRAY[5,6], ARRAY[[1,2],[3,4]]);
--       array_cat
-- ---------------------
--  {{5,6},{1,2},{3,4}}
