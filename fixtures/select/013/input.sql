SELECT * FROM unnest(ARRAY['a','b','c','d','e','f']) WITH ORDINALITY;
--  unnest | ordinality
-- --------+----------
--  a      |        1
--  b      |        2
--  c      |        3
--  d      |        4
--  e      |        5
--  f      |        6
-- (6 rows)
