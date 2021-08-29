SELECT array_dims(ARRAY[1,2] || ARRAY[[3,4],[5,6]]);
--  array_dims
-- ------------
--  [1:3][1:2]
-- (1 row)

SELECT array_dims(ARRAY[1,2] || ARRAY[3,4,5]);
--  array_dims
-- ------------
--  [1:5]
-- (1 row)

SELECT array_dims(ARRAY[[1,2],[3,4]] || ARRAY[[5,6],[7,8],[9,0]]);
--  array_dims
-- ------------
--  [1:5][1:2]
-- (1 row)

SELECT array_dims(1 || '[0:1]={2,3}'::int[]);
 array_dims
------------
 [0:2]
(1 row)

SELECT array_dims(ARRAY[1,2] || 3);
 array_dims
------------
 [1:3]
(1 row)
