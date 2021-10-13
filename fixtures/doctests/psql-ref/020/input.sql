testdb=> \set content `cat my_file.txt`
testdb=> INSERT INTO my_table VALUES (:'content');
