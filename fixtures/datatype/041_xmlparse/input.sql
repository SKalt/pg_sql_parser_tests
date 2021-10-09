select XMLPARSE (DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>');
select XMLPARSE (CONTENT 'abc<foo>bar</foo><bar>foo</bar>');
