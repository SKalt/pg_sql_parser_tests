CREATE TABLE test (col1 integer, col2 text, col3 text);
INSERT INTO test VALUES (123, 'foo', 'bar');

SELECT hstore(t) FROM test AS t;
--                    hstore                    
-- ---------------------------------------------
--  "col1"=>"123", "col2"=>"foo", "col3"=>"bar"
-- (1 row)
