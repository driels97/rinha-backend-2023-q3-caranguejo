CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE OR REPLACE FUNCTION ARRAY_TO_STRING_IMMUTABLE (
    arr TEXT[],
    sep TEXT
) RETURNS TEXT IMMUTABLE PARALLEL SAFE LANGUAGE SQL AS $$
SELECT ARRAY_TO_STRING(arr, sep) $$;

CREATE TABLE IF NOT EXISTS people (
  id uuid PRIMARY KEY,
  apelido VARCHAR (32) NOT NULL UNIQUE,
  nome VARCHAR (100) NOT NULL,
  nascimento DATE NOT NULL,
  stack VARCHAR (32) [],
  search TEXT GENERATED ALWAYS AS (
        nome || ' ' || apelido || ' ' || COALESCE(ARRAY_TO_STRING_IMMUTABLE(stack, ' '), '')
  ) STORED
);

CREATE INDEX people_search_index ON people USING GIN (search gin_trgm_ops);