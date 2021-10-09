SELECT kind, sum(len) AS total
    FROM films
    GROUP BY kind
    HAVING sum(len) < interval '5 hours';

--    kind   | total
-- ----------+-------
--  Comedy   | 02:58
--  Romantic | 04:38
