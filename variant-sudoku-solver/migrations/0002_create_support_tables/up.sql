CREATE TABLE variants (
  id text PRIMARY KEY,
  name text NOT NULL,
  description text,
  metadata jsonb
);

CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  email text UNIQUE,
  display_name text,
  created_at timestamptz DEFAULT now()
);

CREATE TABLE games (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid REFERENCES users(id) ON DELETE SET NULL,
  puzzle_id uuid REFERENCES puzzles(id) ON DELETE CASCADE,
  started_at timestamptz DEFAULT now(),
  completed_at timestamptz,
  time_ms integer,
  moves jsonb,
  result jsonb
);

CREATE INDEX idx_games_user ON games(user_id);
CREATE INDEX idx_games_puzzle ON games(puzzle_id);
