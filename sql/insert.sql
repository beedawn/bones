
INSERT INTO USERS (username, password) VALUES ('user', 'password');
--create a role
INSERT INTO Roletype (role) VALUES ('user');



--update roles table with new role id
INSERT INTO Roles (roletypeid)
SELECT id FROM Roletype WHERE role = 'ruser';


-- Delete existing role assignments for the user 'bee'
DELETE FROM need
WHERE userid = (SELECT id FROM Users WHERE username = 'bee' AND password = 'bee');


-- Example: Assign the 'Administrator' role to user 'johndoe'
INSERT INTO need (userid, roleid) 
VALUES (
    (SELECT id FROM Users WHERE username = 'reg_user' AND password='password'),
    (SELECT id FROM Roles WHERE roletypeid = (SELECT id FROM Roletype WHERE role = 'user'))
);
--also need to remove the old role from need



--select user and role
SELECT u.*, rt.role
FROM users u
JOIN need n ON u.id = n.userid
JOIN roles r ON n.roleid = r.id
JOIN roletype rt ON r.roletypeid = rt.id
WHERE u.username = 'bee' AND u.password = 'bee';



--grants all bills to admin
INSERT INTO Permits (roleid, billid)
SELECT r.id, b.id
FROM Roles r
CROSS JOIN Bills b
WHERE r.roletypeid = (SELECT id FROM Roletype WHERE role = 'admin');


--change roletype name
UPDATE Roletype
SET role = 'SuperUser'
WHERE role = 'Administrator';



--update users role
UPDATE Users
SET role = 'new_role'
WHERE username = 'user_to_update';



-- Grant access to a bill for a user
INSERT INTO Permits (roleid, billid)
VALUES (
    (SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'user'), 
    (SELECT id FROM Bills WHERE id = 2) 
);

-- Revoke access to a bill for a user
DELETE FROM Permits
WHERE roleid IN (SELECT r.id FROM Roles r JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'user') -- Assuming the role is 'User'
AND billid = (SELECT id FROM Bills WHERE billid="1"); -- Replace <bill_condition> with appropriate conditions


SELECT b.*
FROM Bills b
JOIN Permits p ON b.id = p.billid
JOIN Roles r ON p.roleid = r.id
JOIN Roletype rt ON r.roletypeid = rt.id
WHERE rt.role = 'user'; -- Replace 'RoleName' with the specific role name you're interested in

SELECT b.* FROM Bills b JOIN Permits p ON b.id = p.billid JOIN Roles r ON p.roleid = r.id JOIN Roletype rt ON r.roletypeid = rt.id WHERE rt.role = 'Administrator';


