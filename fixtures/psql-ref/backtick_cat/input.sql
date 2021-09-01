\set content `cat my_file.txt`
INSERT INTO my_table VALUES (:'content');
