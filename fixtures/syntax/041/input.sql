SELECT ARRAY(SELECT oid FROM pg_proc WHERE proname LIKE 'bytea%');
--                               array
-- ------------------------------------------------------------------
--  {2011,1954,1948,1952,1951,1244,1950,2005,1949,1953,2006,31,2412}
-- (1 row)

SELECT ARRAY(SELECT ARRAY[i, i*2] FROM generate_series(1,5) AS a(i));
--               array
-- ----------------------------------
--  {{1,2},{2,4},{3,6},{4,8},{5,10}}
-- (1 row)
