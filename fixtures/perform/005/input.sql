CREATE STATISTICS stts3 (mcv) ON city, state FROM zipcodes;

ANALYZE zipcodes;

SELECT m.* FROM pg_statistic_ext join pg_statistic_ext_data on (oid = stxoid),
                pg_mcv_list_items(stxdmcv) m WHERE stxname = 'stts3';

--  index |         values         | nulls | frequency | base_frequency 
-- -------+------------------------+-------+-----------+----------------
--      0 | {Washington, DC}       | {f,f} |  0.003467 |        2.7e-05
--      1 | {Apo, AE}              | {f,f} |  0.003067 |        1.9e-05
--      2 | {Houston, TX}          | {f,f} |  0.002167 |       0.000133
--      3 | {El Paso, TX}          | {f,f} |     0.002 |       0.000113
--      4 | {New York, NY}         | {f,f} |  0.001967 |       0.000114
--      5 | {Atlanta, GA}          | {f,f} |  0.001633 |        3.3e-05
--      6 | {Sacramento, CA}       | {f,f} |  0.001433 |        7.8e-05
--      7 | {Miami, FL}            | {f,f} |    0.0014 |          6e-05
--      8 | {Dallas, TX}           | {f,f} |  0.001367 |        8.8e-05
--      9 | {Chicago, IL}          | {f,f} |  0.001333 |        5.1e-05
--    ...
-- (99 rows)
