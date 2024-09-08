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

# NEW
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

#NEW
CREATE TABLE Notes(
	id SERIAL PRIMARY KEY,
	userid INT,
	note VARCHAR(255),
	billid INT,
	FOREIGN KEY (userid) REFERENCES Users(id),
	FOREIGN KEY (billid) REFERENCES Bills(id)
);

#NEW
CREATE TABLE Comments(
	id SERIAL PRIMARY KEY,
	userid INT,
	noteid INT,
	FOREIGN KEY (userid) REFERENCES Users(id),
	FOREIGN KEY (noteid) REFERENCES Notes(id)
);

#NEW
CREATE TABLE attach(
	id SERIAL PRIMARY KEY,
	billid INT,
	noteid INT,
	FOREIGN KEY (billid) REFERENCES Bills(id),
	FOREIGN KEY (noteid) REFERENCES Notes(id)
);