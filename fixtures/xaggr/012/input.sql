SELECT percentile_disc(0.5) WITHIN GROUP (ORDER BY income) FROM households;
--  percentile_disc
-- -----------------
--            50489
