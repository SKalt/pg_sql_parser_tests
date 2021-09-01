SELECT setval('myseq', 42);           Next nextval will return 43
SELECT setval('myseq', 42, true);     Same as above
SELECT setval('myseq', 42, false);    Next nextval will return 42
