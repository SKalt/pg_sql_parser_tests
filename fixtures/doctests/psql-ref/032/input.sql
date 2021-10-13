testdb=> SELECT first, second, first > 2 AS gt2 FROM my_table;
 first | second | gt2 
-------+--------+-----
     1 | one    | f
     2 | two    | f
     3 | three  | t
     4 | four   | t
(4 rows)

testdb=> \crosstabview first second
 first | one | two | three | four 
-------+-----+-----+-------+------
     1 | f   |     |       | 
     2 |     | f   |       | 
     3 |     |     | t     | 
     4 |     |     |       | t
(4 rows)
