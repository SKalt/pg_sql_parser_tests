CREATE OPERATOR CLASS polygon_ops
    DEFAULT FOR TYPE polygon USING gist AS
        OPERATOR 1 <,
        STORAGE box;
