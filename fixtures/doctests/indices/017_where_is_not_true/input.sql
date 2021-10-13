CREATE INDEX orders_unbilled_index ON orders (order_nr)
    WHERE billed is not true;
