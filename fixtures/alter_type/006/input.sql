CREATE FUNCTION mytypesend(mytype) RETURNS bytea AS '...';
CREATE FUNCTION mytyperecv(internal, oid, integer) RETURNS mytype AS '...';
ALTER TYPE mytype SET (
    SEND = mytypesend,
    RECEIVE = mytyperecv
);
