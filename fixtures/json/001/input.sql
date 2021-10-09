-- SELECT '{"bar": "baz", "balance": 7.77, "active":false}'::json;
--                       json                       
-- -------------------------------------------------
--  {"bar": "baz", "balance": 7.77, "active":false}
-- (1 row)

SELECT '{"bar": "baz", "balance": 7.77, "active":false}'::jsonb;
--                       jsonb                       
-- --------------------------------------------------
--  {"bar": "baz", "active": false, "balance": 7.77}
-- (1 row)
