CREATE TABLE mytable (h hstore);
INSERT INTO mytable VALUES ('a=>b, c=>d');
SELECT h['a'] FROM mytable;
--  h
-- ---
--  b
-- (1 row)

UPDATE mytable SET h['c'] = 'new';
SELECT h FROM mytable;
--           h
-- ----------------------
--  "a"=>"b", "c"=>"new"
-- (1 row)
