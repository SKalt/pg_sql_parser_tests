$plan = spi_prepare('SELECT * FROM test WHERE id > $1 AND name = $2',
                                                     'INTEGER', 'TEXT');
