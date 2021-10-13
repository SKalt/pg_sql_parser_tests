SELECT relname FROM pg_class WHERE pg_table_is_visible(oid);
