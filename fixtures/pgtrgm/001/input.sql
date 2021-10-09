SELECT strict_word_similarity('word', 'two words'), similarity('word', 'words');
--  strict_word_similarity | similarity
-- ------------------------+------------
--                0.571429 |   0.571429
-- (1 row)
