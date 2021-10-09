CREATE OPERATOR CLASS foo
DEFAULT FOR TYPE point
OPERATOR 15    <-> (point, point) FOR ORDER BY float_ops;
