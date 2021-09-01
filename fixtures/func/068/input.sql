SELECT pg_typeof(33);
 pg_typeof
-----------
 integer

SELECT typlen FROM pg_type WHERE oid = pg_typeof(33);
 typlen
--------
      4
