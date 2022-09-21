CREATE TABLE users (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	username VARCHAR(64) NOT NULL UNIQUE, 
    lastname VARCHAR(64), 
    firstname VARCHAR(64),
	email VARCHAR(128), 
	password_hash VARCHAR(128), 
    date_of_birth DATE,
	admin BOOLEAN NOT NULL DEFAULT false,
	active BOOLEAN NOT NULL DEFAULT true
);

-- CREATE TABLE attribute_types (
-- 	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
-- 	name VARCHAR(128) NOT NULL
-- 	datatype VARCHAR(32) NOT NULL
-- );
-- 
-- CREATE TABLE user_attributes (
-- 	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
-- 	user_id BIGINT NOT NULL REFERENCES users, 
-- 	attribute_id BIGINT NOT NULL REFERENCES attribute_types, 
-- );

CREATE TABLE teams (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(1024)
);

CREATE TABLE team_members (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    team_id BIGINT NOT NULL references teams,
	user_id BIGINT NOT NULL references users,
    manager BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE positions (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	team_id BIGINT NOT NULL REFERENCES teams, 
	name VARCHAR(128) NOT NULL,
    date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL
);

-- CREATE TABLE constraint_types (
-- 	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
-- 	name VARCHAR(128) NOT NULL
-- );
-- 
-- CREATE TABLE position_constraints (
-- 	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
--     positiont_id BIGINT NOT NULL,
-- 	constraint_id BIGINT NOT NULL
-- );

CREATE TABLE scheduled_positions (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    position_id BIGINT NOT NULL REFERENCES positions, 
    user_id BIGINT NOT NULL REFERENCES users
);
