SELECT tablename, null_frac,n_distinct, most_common_vals FROM pg_stats
WHERE tablename IN ('tenk1', 'tenk2') AND attname='unique2';

-- tablename  | null_frac | n_distinct | most_common_vals
-- -----------+-----------+------------+------------------
--  tenk1     |         0 |         -1 |
--  tenk2     |         0 |         -1 |
