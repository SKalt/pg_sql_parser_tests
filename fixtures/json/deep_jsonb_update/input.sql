-- Where jsonb_field was {}, it is now {'a': [{'b': 1}]}
UPDATE table_name SET jsonb_field['a'][0]['b'] = '1';

-- Where jsonb_field was [], it is now [null, {'a': 1}]
UPDATE table_name SET jsonb_field[1]['a'] = '1';
