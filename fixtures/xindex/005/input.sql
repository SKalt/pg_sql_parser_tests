CREATE OPERATOR CLASS foo
  FOR TYPE complex
        OPERATOR        1       < (complex, complex) ,
        FUNCTION 1 bar(complex, complex);
