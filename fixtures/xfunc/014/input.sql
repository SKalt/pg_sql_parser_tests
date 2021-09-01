SELECT x, g FROM tab, LATERAL generate_series(1,5) AS g;
