-- Your SQL goes here
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO users (name) VALUES ('Calum');
INSERT INTO users (name) VALUES ('Calum2');
INSERT INTO users (name) VALUES ('Calum3');
INSERT INTO users (name) VALUES ('Calum4');
