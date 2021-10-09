CREATE FUNCTION add(a integer, b integer) RETURNS integer
    AS 'select a + b;'
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT
;
