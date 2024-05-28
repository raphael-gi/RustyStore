CREATE TABLE customer (
  id SERIAL PRIMARY KEY,
  username VARCHAR(45),
  email VARCHAR(90),
  password VARCHAR(90)
);

CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  name VARCHAR(90),
  price MONEY NOT NULL,
  description TEXT
);

INSERT INTO customer (username, email, password)
VALUES ('Chris Bumstead', 'chris@bummy.dev', 'hehe');

INSERT INTO customer (username, email, password)
VALUES ('Kenji', 'kenj@bummy.dev', 'hehe');

CREATE OR REPLACE PROCEDURE register(us VARCHAR, em VARCHAR, password VARCHAR) 
AS $$
DECLARE 
    username_used VARCHAR = NULL;
    email_used VARCHAR = NULL;
BEGIN
  SELECT username, email INTO username_used, email_used FROM customer c WHERE c.username = us OR c.email = em;
  
  IF email_used = em THEN
    RAISE EXCEPTION 'Email "%" is already taken', em USING HINT = 'You might already have an account';
  END IF;
  
  IF username_used = us THEN
    RAISE EXCEPTION 'Username "%" is already taken', us USING HINT = 'Provide another username';
  END IF;
  
  INSERT INTO customer (username, email, password) VALUES (us, em, password);
END;
$$ LANGUAGE plpgsql;

