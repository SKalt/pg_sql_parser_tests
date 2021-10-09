SELECT to_tsvector('fatal error') @@ to_tsquery('fatal <-> error');
--  ?column? 
-- ----------
--  t

SELECT to_tsvector('error is not fatal') @@ to_tsquery('fatal <-> error');
--  ?column? 
-- ----------
--  f
