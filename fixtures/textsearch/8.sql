SELECT title
FROM pgweb
WHERE to_tsvector(body) @@ to_tsquery('friend');
