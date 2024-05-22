CREATE TABLE customer (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  username VARCHAR(45),
  email VARCHAR(90),
  CONSTRAINT customer_pk PRIMARY KEY (id)
);

CREATE TABLE product (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name VARCHAR(90),
  price MONEY NOT NULL,
  description TEXT,
  CONSTRAINT product_pk PRIMARY KEY (id)
);
