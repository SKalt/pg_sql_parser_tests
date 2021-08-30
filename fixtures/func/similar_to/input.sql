select 'abc' SIMILAR TO 'abc'           ; -- true
select 'abc' SIMILAR TO 'a'             ; -- false
select 'abc' SIMILAR TO '%(b|d)%'       ; -- true
select 'abc' SIMILAR TO '(b|c)%'        ; -- false
select '-abc-' SIMILAR TO '%\mabc\M%'   ; -- true
select 'xabcy' SIMILAR TO '%\mabc\M%'   ; -- false
