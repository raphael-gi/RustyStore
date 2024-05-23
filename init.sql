CREATE TABLE customer (
  id SERIAL PRIMARY KEY,
  username VARCHAR(45),
  password
);

CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  name VARCHAR(90),
  price MONEY NOT NULL,
  description TEXT
);

INSERT INTO customer (username, email)
VALUES ('Chris Bumstead', '@christest');

