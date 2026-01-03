INSERT INTO puzzles (name, author, variant, difficulty, board, variant_data)
VALUES (
  'Example Standard Puzzle',
  'seed',
  'standard',
  3,
  '[
    [0,6,0,0,0,0,0,1,0],
    [0,0,0,6,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0]
  ]'::jsonb,
  '{"notes":"seeded puzzle"}'::jsonb
);
