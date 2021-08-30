SELECT extract(julian from '2021-06-23 7:00:00-04'::timestamptz at time zone 'UTC+12');
--            extract
-- ------------------------------
--  2459388.95833333333333333333
-- (1 row)
SELECT extract(julian from '2021-06-23 8:00:00-04'::timestamptz at time zone 'UTC+12');
--                extract
-- --------------------------------------
--  2459389.0000000000000000000000000000
-- (1 row)
SELECT extract(julian from date '2021-06-23');
--  extract
-- ---------
--  2459389
-- (1 row)
