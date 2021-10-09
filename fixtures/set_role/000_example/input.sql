SELECT SESSION_USER, CURRENT_USER;

--  session_user | current_user 
-- --------------+--------------
--  peter        | peter

SET ROLE 'paul';

SELECT SESSION_USER, CURRENT_USER;

--  session_user | current_user 
-- --------------+--------------
--  peter        | paul
