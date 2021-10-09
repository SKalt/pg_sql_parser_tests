SELECT SESSION_USER, CURRENT_USER;

--  session_user | current_user 
-- --------------+--------------
--  peter        | peter

SET SESSION AUTHORIZATION 'paul';

SELECT SESSION_USER, CURRENT_USER;

--  session_user | current_user 
-- --------------+--------------
--  paul         | paul
