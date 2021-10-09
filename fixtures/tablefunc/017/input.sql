CREATE TABLE connectby_tree(keyid text, parent_keyid text, pos int);

INSERT INTO connectby_tree VALUES('row1',NULL, 0);
INSERT INTO connectby_tree VALUES('row2','row1', 0);
INSERT INTO connectby_tree VALUES('row3','row1', 0);
INSERT INTO connectby_tree VALUES('row4','row2', 1);
INSERT INTO connectby_tree VALUES('row5','row2', 0);
INSERT INTO connectby_tree VALUES('row6','row4', 0);
INSERT INTO connectby_tree VALUES('row7','row3', 0);
INSERT INTO connectby_tree VALUES('row8','row6', 0);
INSERT INTO connectby_tree VALUES('row9','row5', 0);

-- with branch, without orderby_fld (order of results is not guaranteed)
SELECT * FROM connectby('connectby_tree', 'keyid', 'parent_keyid', 'row2', 0, '~')
 AS t(keyid text, parent_keyid text, level int, branch text);
--  keyid | parent_keyid | level |       branch
-- -------+--------------+-------+---------------------
--  row2  |              |     0 | row2
--  row4  | row2         |     1 | row2~row4
--  row6  | row4         |     2 | row2~row4~row6
--  row8  | row6         |     3 | row2~row4~row6~row8
--  row5  | row2         |     1 | row2~row5
--  row9  | row5         |     2 | row2~row5~row9
-- (6 rows)

-- without branch, without orderby_fld (order of results is not guaranteed)
SELECT * FROM connectby('connectby_tree', 'keyid', 'parent_keyid', 'row2', 0)
 AS t(keyid text, parent_keyid text, level int);
--  keyid | parent_keyid | level
-- -------+--------------+-------
--  row2  |              |     0
--  row4  | row2         |     1
--  row6  | row4         |     2
--  row8  | row6         |     3
--  row5  | row2         |     1
--  row9  | row5         |     2
-- (6 rows)

-- with branch, with orderby_fld (notice that row5 comes before row4)
SELECT * FROM connectby('connectby_tree', 'keyid', 'parent_keyid', 'pos', 'row2', 0, '~')
 AS t(keyid text, parent_keyid text, level int, branch text, pos int);
--  keyid | parent_keyid | level |       branch        | pos
-- -------+--------------+-------+---------------------+-----
--  row2  |              |     0 | row2                |   1
--  row5  | row2         |     1 | row2~row5           |   2
--  row9  | row5         |     2 | row2~row5~row9      |   3
--  row4  | row2         |     1 | row2~row4           |   4
--  row6  | row4         |     2 | row2~row4~row6      |   5
--  row8  | row6         |     3 | row2~row4~row6~row8 |   6
-- (6 rows)

-- without branch, with orderby_fld (notice that row5 comes before row4)
SELECT * FROM connectby('connectby_tree', 'keyid', 'parent_keyid', 'pos', 'row2', 0)
 AS t(keyid text, parent_keyid text, level int, pos int);
--  keyid | parent_keyid | level | pos
-- -------+--------------+-------+-----
--  row2  |              |     0 |   1
--  row5  | row2         |     1 |   2
--  row9  | row5         |     2 |   3
--  row4  | row2         |     1 |   4
--  row6  | row4         |     2 |   5
--  row8  | row6         |     3 |   6
-- (6 rows)
