CREATE FUNCTION foo() RETURNS integer AS $f$
  a_output := a_output || $$ if v_$$ || referrer_keys.kind || $$ like '$$
      || referrer_keys.key_string || $$'
      then return '$$  || referrer_keys.referrer_type
      || $$'; end if;$$;
$f$ LANGUAGE plpgsql;
