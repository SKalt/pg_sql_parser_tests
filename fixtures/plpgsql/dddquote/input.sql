CREATE FUNCTION foo() RETURNS integer AS '
  a_output := a_output || '' AND name LIKE ''''foobar'''' AND xyz''
' LANGUAGE plpgsql;
