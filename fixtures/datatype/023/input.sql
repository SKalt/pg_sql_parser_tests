SELECT person.name, holidays.num_weeks FROM person, holidays
  WHERE person.current_mood::text = holidays.happiness::text;
--  name | num_weeks 
-- ------+-----------
--  Moe  |         4
-- (1 row)
