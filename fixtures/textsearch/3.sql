SELECT 'fat cats ate fat rats'::tsvector @@ to_tsquery('fat & rat');
 ?column? 
----------
 f
