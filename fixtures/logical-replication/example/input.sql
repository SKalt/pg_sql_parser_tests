CREATE PUBLICATION mypub FOR TABLE users, departments;
CREATE SUBSCRIPTION mysub CONNECTION 'dbname=foo host=bar user=repuser' PUBLICATION mypub;
