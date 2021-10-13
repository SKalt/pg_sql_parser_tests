CREATE UNIQUE INDEX tests_success_constraint ON tests (subject, target)
    WHERE success;
