CREATE TYPE box;

CREATE FUNCTION my_box_in_function(cstring) RETURNS box AS '...' LANGUAGE SQL ;
CREATE FUNCTION my_box_out_function(box) RETURNS cstring AS '...' LANGUAGE SQL ;

CREATE TYPE box (
    INTERNALLENGTH = 16,
    INPUT = my_box_in_function,
    OUTPUT = my_box_out_function
);

CREATE TABLE myboxes (
    id integer,
    description box
);
