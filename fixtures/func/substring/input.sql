select substring('foobar' from '%#"o_b#"%' for '#')   ; -- oob
select substring('foobar' from '#"o_b#"%' for '#')    ; -- NULL
