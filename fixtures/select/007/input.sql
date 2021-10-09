SELECT f.title, f.did, d.name, f.date_prod, f.kind
    FROM distributors d, films f
    WHERE f.did = d.did

--        title       | did |     name     | date_prod  |   kind
-- -------------------+-----+--------------+------------+----------
--  The Third Man     | 101 | British Lion | 1949-12-23 | Drama
--  The African Queen | 101 | British Lion | 1951-08-11 | Romantic
--  ...
