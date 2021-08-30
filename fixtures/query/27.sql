SELECT city, max(temp_lo)
    FROM weather
    WHERE city LIKE 'S%'            -- 