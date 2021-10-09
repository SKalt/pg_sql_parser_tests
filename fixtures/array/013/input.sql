SELECT schedule[:2][2:] FROM sal_emp WHERE name = 'Bill';

--         schedule
-- ------------------------
--  {{lunch},{presentation}}
-- (1 row)

SELECT schedule[:][1:1] FROM sal_emp WHERE name = 'Bill';

--         schedule
-- ------------------------
--  {{meeting},{training}}
-- (1 row)
