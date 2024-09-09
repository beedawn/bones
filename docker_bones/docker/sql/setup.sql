CREATE TABLE Users (
	id SERIAL PRIMARY KEY,
	username VARCHAR(255),
	password VARCHAR(255)
);

CREATE TABLE Roletype(
	id SERIAL PRIMARY KEY,
	role VARCHAR(255)

);

CREATE TABLE Roles (
	id SERIAL PRIMARY KEY,
	roletypeid INT,
	FOREIGN KEY (roletypeid) REFERENCES Roletype(id)
);

CREATE TABLE need(
	id SERIAL PRIMARY KEY,
	userid INT,
	roleid INT,
	FOREIGN KEY (userid) REFERENCES Users(id),
	FOREIGN KEY (roleid) REFERENCES Roles(id)
);

CREATE TABLE BillStatus(
	id SERIAL PRIMARY KEY,
	status VARCHAR(255)
);

CREATE TABLE Provider(
	id SERIAL PRIMARY KEY,
	url VARCHAR(255),
	phone VARCHAR(255),
	name VARCHAR(255)
);

CREATE TABLE Bills(
	id SERIAL PRIMARY KEY,
	amount double precision,
	date VARCHAR(255),
	img_path VARCHAR(255),
	duedate VARCHAR(255),
	providerid INT,
	billstatusid INT,
	FOREIGN KEY (billstatusid) REFERENCES BillStatus(id),
	FOREIGN KEY (providerid) REFERENCES Provider(id)
);

CREATE TABLE Permits(
	id SERIAL PRIMARY KEY,
	roleid INT,
	billid INT,
	FOREIGN KEY (roleid) REFERENCES Roles(id),
	FOREIGN KEY (billid) REFERENCES Bills(id)
);

 
CREATE TABLE Notes(
	id SERIAL PRIMARY KEY,
	userid INT,
	note VARCHAR(255),
	billid INT,
	FOREIGN KEY (userid) REFERENCES Users(id),
	FOREIGN KEY (billid) REFERENCES Bills(id)
);

 
CREATE TABLE Comments(
	id SERIAL PRIMARY KEY,
	userid INT,
	noteid INT,
	FOREIGN KEY (userid) REFERENCES Users(id),
	FOREIGN KEY (noteid) REFERENCES Notes(id)
);

 
CREATE TABLE attach(
	id SERIAL PRIMARY KEY,
	billid INT,
	noteid INT,
	FOREIGN KEY (billid) REFERENCES Bills(id),
	FOREIGN KEY (noteid) REFERENCES Notes(id)
);


--Insert initial user
INSERT INTO USERS (username, password) VALUES ('username', 'password') RETURNING id;
INSERT INTO Roletype (role) VALUES ('admin');
INSERT INTO roles (id, roletypeid) VALUES (1, 1);
--add role to user, 1 is admin for initial user
INSERT INTO need (userid, roleid) VALUES ((SELECT id FROM Users WHERE username = 'username' AND password='password'),('1'));
