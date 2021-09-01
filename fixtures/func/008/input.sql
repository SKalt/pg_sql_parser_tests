'abcd' ~ 'bc'     true
'abcd' ~ 'a.c'    true &mdash; dot matches any character
'abcd' ~ 'a.*d'   true &mdash; * repeats the preceding pattern item
'abcd' ~ '(b|x)'  true &mdash; | means OR, parentheses group
'abcd' ~ '^a'     true &mdash; ^ anchors to start of string
'abcd' ~ '^(b|c)' false &mdash; would match except for anchoring
