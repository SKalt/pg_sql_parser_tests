<<block>>
DECLARE
    foo int;
BEGIN
    foo := '123';
    INSERT INTO dest (col) SELECT block.foo + bar FROM src;
