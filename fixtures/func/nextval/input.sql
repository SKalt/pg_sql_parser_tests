select nextval('foo')      ; -- operates on sequence foo
select nextval('FOO')      ; -- operates on sequence foo
select nextval('"Foo"')    ; -- operates on sequence Foo

-- The sequence name can be schema-qualified if necessary:

select nextval('myschema.foo')     ; -- operates on myschema.foo
select nextval('"myschema".foo')   ; -- same as above
select nextval('foo')              ; -- searches search path for foo
