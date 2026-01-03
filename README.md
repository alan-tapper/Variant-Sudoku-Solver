# Variant-Sudoku-Solver
Sudoku solver that handles variants such as those seen on Cracking the Cryptic. Includes rule highlighting, hints, and import/export of board states.

# Quick Start
1. Install Rust and Cargo from https://rustup.rs/
2. Clone this repository
3. Set up a PostgreSQL database and set the `DATABASE_URL` environment variable to point
    to it (e.g. `export DATABASE_URL=postgres://user:password@localhost/variant_sudoku`)
4. Run database migrations with `cargo install sqlx-cli` followed by `sqlx migrate run`
5. (Optional) Import sample boards with `cargo run -- import-samples`
6. Start the server with `cargo run -- serve`
7. Access the web interface at `http://localhost:8000`

# Features:
1. Rule highlighting
2. Hints
3. Import/Export of board states
4. Solving algorithm that can handle multiple variants at once

## Variants supported
1. Anti-Knight Sudoku
2. Anti-King Sudoku
3. Anti-Diagonal Sudoku

## Combinations of Variants
Most combinations of the variants are supported. The puzzle then has the restrictions from multiple variants at once.