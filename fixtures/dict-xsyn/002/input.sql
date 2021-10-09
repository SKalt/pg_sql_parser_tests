SELECT ts_lexize('xsyn', 'word');
--       ts_lexize
-- -----------------------
--  {syn1,syn2,syn3}

ALTER TEXT SEARCH DICTIONARY xsyn (RULES='my_rules', KEEPORIG=true);
-- ALTER TEXT SEARCH DICTIONARY

SELECT ts_lexize('xsyn', 'word');
--       ts_lexize
-- -----------------------
--  {word,syn1,syn2,syn3}

ALTER TEXT SEARCH DICTIONARY xsyn (RULES='my_rules', KEEPORIG=false, MATCHSYNONYMS=true);
-- ALTER TEXT SEARCH DICTIONARY

SELECT ts_lexize('xsyn', 'syn1');
--       ts_lexize
-- -----------------------
--  {syn1,syn2,syn3}

ALTER TEXT SEARCH DICTIONARY xsyn (RULES='my_rules', KEEPORIG=true, MATCHORIG=false, KEEPSYNONYMS=false);
-- ALTER TEXT SEARCH DICTIONARY

SELECT ts_lexize('xsyn', 'syn1');
--       ts_lexize
-- -----------------------
--  {word}
