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
