CREATE FUNCTION raise_custom_exception() RETURNS void AS $$
plpy.error("custom exception message",
           detail="some info about exception",
           hint="hint for users")
$$ LANGUAGE plpythonu;

SELECT raise_custom_exception();
-- ERROR:  plpy.Error: custom exception message
-- DETAIL:  some info about exception
-- HINT:  hint for users
-- CONTEXT:  Traceback (most recent call last):
--   PL/Python function "raise_custom_exception", line 4, in <module>
--     hint="hint for users")
-- PL/Python function "raise_custom_exception"
