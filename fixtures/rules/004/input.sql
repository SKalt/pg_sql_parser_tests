INSERT INTO unit VALUES ('cm', 1.0);
INSERT INTO unit VALUES ('m', 100.0);
INSERT INTO unit VALUES ('inch', 2.54);

INSERT INTO shoe_data VALUES ('sh1', 2, 'black', 70.0, 90.0, 'cm');
INSERT INTO shoe_data VALUES ('sh2', 0, 'black', 30.0, 40.0, 'inch');
INSERT INTO shoe_data VALUES ('sh3', 4, 'brown', 50.0, 65.0, 'cm');
INSERT INTO shoe_data VALUES ('sh4', 3, 'brown', 40.0, 50.0, 'inch');

INSERT INTO shoelace_data VALUES ('sl1', 5, 'black', 80.0, 'cm');
INSERT INTO shoelace_data VALUES ('sl2', 6, 'black', 100.0, 'cm');
INSERT INTO shoelace_data VALUES ('sl3', 0, 'black', 35.0 , 'inch');
INSERT INTO shoelace_data VALUES ('sl4', 8, 'black', 40.0 , 'inch');
INSERT INTO shoelace_data VALUES ('sl5', 4, 'brown', 1.0 , 'm');
INSERT INTO shoelace_data VALUES ('sl6', 0, 'brown', 0.9 , 'm');
INSERT INTO shoelace_data VALUES ('sl7', 7, 'brown', 60 , 'cm');
INSERT INTO shoelace_data VALUES ('sl8', 1, 'brown', 40 , 'inch');

SELECT * FROM shoelace;

--  sl_name   | sl_avail | sl_color | sl_len | sl_unit | sl_len_cm
-- -----------+----------+----------+--------+---------+-----------
--  sl1       |        5 | black    |     80 | cm      |        80
--  sl2       |        6 | black    |    100 | cm      |       100
--  sl7       |        7 | brown    |     60 | cm      |        60
--  sl3       |        0 | black    |     35 | inch    |      88.9
--  sl4       |        8 | black    |     40 | inch    |     101.6
--  sl8       |        1 | brown    |     40 | inch    |     101.6
--  sl5       |        4 | brown    |      1 | m       |       100
--  sl6       |        0 | brown    |    0.9 | m       |        90
-- (8 rows)
