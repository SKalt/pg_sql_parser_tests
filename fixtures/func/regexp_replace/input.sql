select regexp_replace('foobarbaz', 'b..', 'X');
                                  --  fooXbaz
select regexp_replace('foobarbaz', 'b..', 'X', 'g');
                                  --  fooXX
select regexp_replace('foobarbaz', 'b(..)', 'X\1Y', 'g');
                                  --  fooXarYXazY
