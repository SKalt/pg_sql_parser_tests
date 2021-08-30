SELECT to_tsvector( 'postgraduate' ) @@ to_tsquery( 'postgres:*' );
--  ?column?
-- ----------
--  t
