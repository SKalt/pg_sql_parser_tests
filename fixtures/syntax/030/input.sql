SELECT
    count(*) AS unfiltered,
    count(*) FILTER (WHERE i < 5) AS filtered
FROM generate_series(1,10) AS s(i);
--  unfiltered | filtered
-- ------------+----------
--          10 |        4
-- (1 row)
