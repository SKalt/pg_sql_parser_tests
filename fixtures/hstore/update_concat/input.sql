UPDATE tab SET h = h || hstore('c', '3');
UPDATE tab SET h = h || hstore(array['q', 'w'], array['11', '12']);
