SELECT relname FROM pg_class WHERE pg_table_is_visible(oid);
SELECT pg_type_is_visible('myschema.widget'::regtype);
