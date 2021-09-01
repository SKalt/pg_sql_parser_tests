SELECT depname, empno, salary, avg(salary) OVER (PARTITION BY depname) FROM empsalary;
