\pset border 2
-- Border style is 2.
SELECT * FROM my_table;
-- +-------+--------+
-- | first | second |
-- +-------+--------+
-- |     1 | one    |
-- |     2 | two    |
-- |     3 | three  |
-- |     4 | four   |
-- +-------+--------+
-- (4 rows)

\pset border 0
-- Border style is 0.
SELECT * FROM my_table;
-- first second
-- ----- ------
--     1 one
--     2 two
--     3 three
--     4 four
-- (4 rows)

\pset border 1
-- Border style is 1.
\pset format csv
-- Output format is csv.
\pset tuples_only
-- Tuples only is on.
SELECT second, first FROM my_table;
-- one,1
-- two,2
-- three,3
-- four,4
\pset format unaligned
-- Output format is unaligned.
\pset fieldsep '\t'
-- Field separator is "    ".
SELECT second, first FROM my_table;
-- one     1
-- two     2
-- three   3
-- four    4
