BEGIN;
CREATE FUNCTION check_password(uname TEXT, pass TEXT) AS $$ ... $$ language plpgsql SECURITY DEFINER;
REVOKE ALL ON FUNCTION check_password(uname TEXT, pass TEXT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION check_password(uname TEXT, pass TEXT) TO admins;
COMMIT;
