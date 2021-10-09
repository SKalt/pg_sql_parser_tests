CREATE STATISTICS stts (dependencies) ON city, zip FROM zipcodes;

ANALYZE zipcodes;

SELECT stxname, stxkeys, stxddependencies
  FROM pg_statistic_ext join pg_statistic_ext_data on (oid = stxoid)
  WHERE stxname = 'stts';
--  stxname | stxkeys |             stxddependencies             
-- ---------+---------+------------------------------------------
--  stts    | 1 5     | {"1 => 5": 1.000000, "5 => 1": 0.423130}
-- (1 row)
