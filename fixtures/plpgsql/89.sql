RAISE unique_violation USING MESSAGE = 'Duplicate user ID: ' || user_id;
