SELECT ARRAY[1, 2] || '{3, 4}';  -- the untyped literal is taken as an array
--  ?column?
-- -----------
--  {1,2,3,4}

SELECT ARRAY[1, 2] || '7';                 -- so is this one
ERROR:  malformed array literal: "7"

SELECT ARRAY[1, 2] || NULL;                -- so is an undecorated NULL
--  ?column?
-- ----------
--  {1,2}
-- (1 row)

SELECT array_append(ARRAY[1, 2], NULL);    -- this might have been meant
--  array_append
-- --------------
--  {1,2,NULL}

SELECT ARRAY[1,2] || ARRAY[3,4];
--  ?column?
-- -----------
--  {1,2,3,4}
-- (1 row)

SELECT ARRAY[5,6] || ARRAY[[1,2],[3,4]];
--       ?column?
-- ---------------------
--  {{5,6},{1,2},{3,4}}
-- (1 row)
