SELECT 'fat & rat'::tsquery;
--     tsquery    
-- ---------------
--  'fat' & 'rat'

SELECT 'fat & (rat | cat)'::tsquery;
--           tsquery          
-- ---------------------------
--  'fat' & ( 'rat' | 'cat' )

SELECT 'fat & rat & ! cat'::tsquery;
--         tsquery         
-- ------------------------
--  'fat' & 'rat' & !'cat'
SELECT 'fat:ab & cat'::tsquery;
--     tsquery
-- ------------------
--  'fat':AB & 'cat'
SELECT 'super:*'::tsquery;
--   tsquery  
-- -----------
--  'super':*
