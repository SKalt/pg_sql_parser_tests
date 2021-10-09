SELECT ROW(1,2.5,'this is a test') = ROW(1, 3, 'not the same');

SELECT ROW(tble.*) IS NULL FROM tble;  -- detect all-null rows
