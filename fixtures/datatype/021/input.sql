INSERT INTO person VALUES ('Larry', 'sad');
INSERT INTO person VALUES ('Curly', 'ok');
SELECT * FROM person WHERE current_mood > 'sad';
--  name  | current_mood 
-- -------+--------------
--  Moe   | happy
--  Curly | ok
-- (2 rows)

SELECT * FROM person WHERE current_mood > 'sad' ORDER BY current_mood;
--  name  | current_mood 
-- -------+--------------
--  Curly | ok
--  Moe   | happy
-- (2 rows)

SELECT name
FROM person
WHERE current_mood = (SELECT MIN(current_mood) FROM person);
--  name  
-- -------
--  Larry
-- (1 row)
