SELECT * FROM pg_create_physical_replication_slot('node_a_slot');
--   slot_name  | lsn
-- -------------+-----
--  node_a_slot |

SELECT slot_name, slot_type, active FROM pg_replication_slots;
--   slot_name  | slot_type | active 
-- -------------+-----------+--------
--  node_a_slot | physical  | f
-- (1 row)
