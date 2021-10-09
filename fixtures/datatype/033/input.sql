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
