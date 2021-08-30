
SELECT foo FROM regexp_split_to_table('the quick brown fox jumps over the lazy dog', '\s+') AS foo;
--   foo   
-- -------
--  the    
--  quick  
--  brown  
--  fox    
--  jumps 
--  over   
--  the    
--  lazy   
--  dog    
-- (9 rows)

SELECT regexp_split_to_array('the quick brown fox jumps over the lazy dog', '\s+');
--               regexp_split_to_array             
-- -----------------------------------------------
--  {the,quick,brown,fox,jumps,over,the,lazy,dog}
-- (1 row)

SELECT foo FROM regexp_split_to_table('the quick brown fox', '\s*') AS foo;
--  foo 
-- -----
--  t         
--  h         
--  e         
--  q         
--  u         
--  i         
--  c         
--  k         
--  b         
--  r         
--  o         
--  w         
--  n         
--  f         
--  o         
--  x         
-- (16 rows)
