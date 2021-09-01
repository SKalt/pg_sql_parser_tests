CREATE FUNCTION foo() RETURNS integer AS '
  a_output := ''Blah'';
  SELECT * FROM users WHERE f_name=''foobar'';
' LANGUAGE plpgsql;
