SELECT t.*,i.doc_num FROM
  xpath_table('id', 'xml', 'test',
              '/doc/line/@num|/doc/line/a|/doc/line/b|/doc/line/c',
              'true')
    AS t(id int, line_num varchar(10), val1 int, val2 int, val3 int),
  xpath_table('id', 'xml', 'test', '/doc/@num', 'true')
    AS i(id int, doc_num varchar(10))
WHERE i.id=t.id AND i.id=1
ORDER BY doc_num, line_num;

--  id | line_num | val1 | val2 | val3 | doc_num
-- ----+----------+------+------+------+---------
--   1 | L1       |    1 |    2 |    3 | C1
--   1 | L2       |   11 |   22 |   33 | C1
-- (2 rows)
