CREATE FUNCTION reffunc2() RETURNS refcursor AS '
DECLARE
    ref refcursor;
BEGIN
    OPEN ref FOR SELECT col FROM test;
    RETURN ref;
END;
' LANGUAGE plpgsql;

-- need to be in a transaction to use cursors.
BEGIN;
SELECT reffunc2();

--       reffunc2
-- --------------------
--  <unnamed cursor 1>
-- (1 row)

FETCH ALL IN "<unnamed cursor 1>";
COMMIT;
