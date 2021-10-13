SELECT salary, sum(salary) OVER (ORDER BY salary) FROM empsalary;
