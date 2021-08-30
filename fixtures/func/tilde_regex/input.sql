select 'abcd' ~ 'bc'     ; -- true
select 'abcd' ~ 'a.c'    ; -- true — dot matches any character
select 'abcd' ~ 'a.*d'   ; -- true — * repeats the preceding pattern item
select 'abcd' ~ '(b|x)'  ; -- true — | means OR, parentheses group
select 'abcd' ~ '^a'     ; -- true — ^ anchors to start of string
select 'abcd' ~ '^(b|c)' ; -- false — would match except for anchoring
