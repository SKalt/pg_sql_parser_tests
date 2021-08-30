select 'abc' LIKE 'abc'    ; -- true
select 'abc' LIKE 'a%'     ; -- true
select 'abc' LIKE '_b_'    ; -- true
select 'abc' LIKE 'c'      ; -- false
