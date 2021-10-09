SELECT EXTRACT(hours from '80 minutes'::interval);
--  date_part
-- -----------
--          1

SELECT EXTRACT(days from '80 hours'::interval);
--  date_part
-- -----------
--          0
