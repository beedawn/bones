--Insert initial user
INSERT INTO USERS (username, password) VALUES ('username', 'password') RETURNING id


--add role to user, 1 is admin for initial user
INSERT INTO need (userid, roleid) VALUES ((SELECT id FROM Users WHERE username = 'username' AND password='password'),('1'));
