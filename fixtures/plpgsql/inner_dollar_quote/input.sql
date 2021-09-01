CREATE FUNCTION foo() RETURNS integer AS $fn$
  a_output := a_output || $$ AND name LIKE 'foobar' AND xyz$$
$fn$ LANGUAGE plpgsql;
