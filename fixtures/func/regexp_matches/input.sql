SELECT regexp_matches('foo', 'not there');
--  regexp_matches
-- ----------------
-- (0 rows)

SELECT regexp_matches('foobarbequebazilbarfbonk', '(b[^b]+)(b[^b]+)', 'g');
--  regexp_matches
-- ----------------
--  {bar,beque}
--  {bazil,barf}
-- (2 rows)
