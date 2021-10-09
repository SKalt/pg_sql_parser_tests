CREATE TEXT SEARCH CONFIGURATION fr ( COPY = french );
ALTER TEXT SEARCH CONFIGURATION fr
        ALTER MAPPING FOR hword, hword_part, word
        WITH unaccent, french_stem;
select to_tsvector('fr','H&ocirc;tels de la Mer');
--     to_tsvector
-- -------------------
--  'hotel':1 'mer':4
-- (1 row)

select to_tsvector('fr','H&ocirc;tel de la Mer') @@ to_tsquery('fr','Hotels');
--  ?column?
-- ----------
--  t
-- (1 row)

select ts_headline('fr','H&ocirc;tel de la Mer',to_tsquery('fr','Hotels'));
--       ts_headline
-- ------------------------
--  <b>H&ocirc;tel</b> de la Mer
-- (1 row)
