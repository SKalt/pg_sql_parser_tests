SELECT to_tsvector('fat cats ate fat rats') @@ to_tsquery('fat & rat');
 ?column? 
----------
 t
