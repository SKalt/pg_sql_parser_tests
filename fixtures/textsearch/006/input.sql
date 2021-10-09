SELECT phraseto_tsquery('cats ate rats');
--        phraseto_tsquery        
-- -------------------------------
--  'cat' <-> 'ate' <-> 'rat'

SELECT phraseto_tsquery('the cats ate the rats');
--        phraseto_tsquery        
-- -------------------------------
--  'cat' <-> 'ate' <2> 'rat'
