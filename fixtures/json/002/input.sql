SELECT '{"reading": 1.230e-5}'::json, '{"reading": 1.230e-5}'::jsonb;
--          json          |          jsonb          
-- -----------------------+-------------------------
--  {"reading": 1.230e-5} | {"reading": 0.00001230}
-- (1 row)
