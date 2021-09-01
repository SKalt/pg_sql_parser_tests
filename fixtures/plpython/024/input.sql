plan = plpy.prepare("SELECT last_name FROM my_users WHERE first_name = $1", ["text"])
