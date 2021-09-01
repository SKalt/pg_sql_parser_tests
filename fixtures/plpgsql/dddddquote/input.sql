CREATE FUNCTION foo() RETURNS integer AS '
  a_output := a_output || '' if v_'' ||
      referrer_keys.kind || '' like ''''''''''
      || referrer_keys.key_string || ''''''''''
      then return ''''''  || referrer_keys.referrer_type
      || ''''''; end if;'';
' LANGUAGE plpgsql;
