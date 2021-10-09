select nextval('foo');             -- operates on sequence foo
select nextval('FOO');             -- same as above
select nextval('"Foo"');           -- operates on sequence Foo
select nextval('myschema.foo');    -- operates on myschema.foo
select nextval('"myschema".foo');  -- same as above
select nextval('foo');             -- searches search path for foo
