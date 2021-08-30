-- Create a slot named 'regression_slot' using the output plugin 'test_decoding'
SELECT * FROM pg_create_logical_replication_slot('regression_slot', 'test_decoding', false, true);
    slot_name    |    lsn
-----------------+-----------
 regression_slot | 0/16B1970
(1 row)

SELECT slot_name, plugin, slot_type, database, active, restart_lsn, confirmed_flush_lsn FROM pg_replication_slots;
    slot_name    |    plugin     | slot_type | database | active | restart_lsn | confirmed_flush_lsn
-----------------+---------------+-----------+----------+--------+-------------+-----------------
 regression_slot | test_decoding | logical   | postgres | f      | 0/16A4408   | 0/16A4440
(1 row)

-- There are no changes to see yet
SELECT * FROM pg_logical_slot_get_changes('regression_slot', NULL, NULL);
--  lsn | xid | data 
-- -----+-----+------
-- (0 rows)

CREATE TABLE data(id serial primary key, data text);
-- CREATE TABLE
-- DDL isn't replicated, so all you'll see is the transaction
SELECT * FROM pg_logical_slot_get_changes('regression_slot', NULL, NULL);
--     lsn    |  xid  |     data     
-- -----------+-------+--------------
--  0/BA2DA58 | 10297 | BEGIN 10297
--  0/BA5A5A0 | 10297 | COMMIT 10297
-- (2 rows)

-- Once changes are read, they're consumed and not emitted
-- in a subsequent call:
SELECT * FROM pg_logical_slot_get_changes('regression_slot', NULL, NULL);
--  lsn | xid | data 
-- -----+-----+------
-- (0 rows)

BEGIN;
INSERT INTO data(data) VALUES('1');
INSERT INTO data(data) VALUES('2');
COMMIT;

SELECT * FROM pg_logical_slot_get_changes('regression_slot', NULL, NULL);
--     lsn    |  xid  |                          data                           
-- -----------+-------+---------------------------------------------------------
--  0/BA5A688 | 10298 | BEGIN 10298
--  0/BA5A6F0 | 10298 | table public.data: INSERT: id[integer]:1 data[text]:'1'
--  0/BA5A7F8 | 10298 | table public.data: INSERT: id[integer]:2 data[text]:'2'
--  0/BA5A8A8 | 10298 | COMMIT 10298
-- (4 rows)

INSERT INTO data(data) VALUES('3');

-- You can also peek ahead in the change stream without consuming changes
SELECT * FROM pg_logical_slot_peek_changes('regression_slot', NULL, NULL);
--     lsn    |  xid  |                          data                           
-- -----------+-------+---------------------------------------------------------
--  0/BA5A8E0 | 10299 | BEGIN 10299
--  0/BA5A8E0 | 10299 | table public.data: INSERT: id[integer]:3 data[text]:'3'
--  0/BA5A990 | 10299 | COMMIT 10299
-- (3 rows)

-- The next call to pg_logical_slot_peek_changes() returns the same changes again
SELECT * FROM pg_logical_slot_peek_changes('regression_slot', NULL, NULL);
--     lsn    |  xid  |                          data                           
-- -----------+-------+---------------------------------------------------------
--  0/BA5A8E0 | 10299 | BEGIN 10299
--  0/BA5A8E0 | 10299 | table public.data: INSERT: id[integer]:3 data[text]:'3'
--  0/BA5A990 | 10299 | COMMIT 10299
-- (3 rows)

-- options can be passed to output plugin, to influence the formatting
SELECT * FROM pg_logical_slot_peek_changes('regression_slot', NULL, NULL, 'include-timestamp', 'on');
--     lsn    |  xid  |                          data                           
-- -----------+-------+---------------------------------------------------------
--  0/BA5A8E0 | 10299 | BEGIN 10299
--  0/BA5A8E0 | 10299 | table public.data: INSERT: id[integer]:3 data[text]:'3'
--  0/BA5A990 | 10299 | COMMIT 10299 (at 2017-05-10 12:07:21.272494-04)
-- (3 rows)

-- Remember to destroy a slot you no longer need to stop it consuming
-- server resources:
SELECT pg_drop_replication_slot('regression_slot');
--  pg_drop_replication_slot
-- -----------------------

-- (1 row)
