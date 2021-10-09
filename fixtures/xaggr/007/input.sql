SELECT attrelid::regclass, array_accum(attname)
    FROM pg_attribute
    WHERE attnum > 0 AND attrelid = 'pg_tablespace'::regclass
    GROUP BY attrelid;

--    attrelid    |              array_accum              
-- ---------------+---------------------------------------
--  pg_tablespace | {spcname,spcowner,spcacl,spcoptions}
-- (1 row)

SELECT attrelid::regclass, array_accum(atttypid::regtype)
    FROM pg_attribute
    WHERE attnum > 0 AND attrelid = 'pg_tablespace'::regclass
    GROUP BY attrelid;

--    attrelid    |        array_accum        
-- ---------------+---------------------------
--  pg_tablespace | {name,oid,aclitem[],text[]}
-- (1 row)
