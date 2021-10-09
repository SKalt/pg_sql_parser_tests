/* peter@localhost testdb=> */ SELECT * FROM my_table
/* peter@localhost testdb-> */ \g (format=aligned tuples_only=off expanded=on)
-- -[ RECORD 1 ]-
-- first  | 1
-- second | one
-- -[ RECORD 2 ]-
-- first  | 2
-- second | two
-- -[ RECORD 3 ]-
-- first  | 3
-- second | three
-- -[ RECORD 4 ]-
-- first  | 4
-- second | four
