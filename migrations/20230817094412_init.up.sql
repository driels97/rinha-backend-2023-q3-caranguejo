CREATE TABLE IF NOT EXISTS people (
  id uuid PRIMARY KEY,
  apelido VARCHAR (32) NOT NULL UNIQUE,
  nome VARCHAR (100) NOT NULL,
  nascimento DATE NOT NULL,
  stack TEXT []
);