-- create extension (Supabase provides this; safe to include)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE puzzles (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  author text,
  variant text NOT NULL,
  difficulty smallint,
  board jsonb NOT NULL,
  variant_data jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_puzzles_variant ON puzzles (variant);
CREATE INDEX idx_puzzles_difficulty ON puzzles (difficulty);
CREATE INDEX idx_puzzles_created_at ON puzzles (created_at);
CREATE INDEX idx_puzzles_variant_data_gin ON puzzles USING gin (variant_data);
