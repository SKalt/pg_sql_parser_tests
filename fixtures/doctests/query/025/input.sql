SELECT city, max(temp_lo)
    FROM weather
    GROUP BY city;
