mod game;
use game::Game;
use std::env;
mod db;
mod local_board_importer;
mod variants;
mod models;
mod importer;

use axum::{routing::get, Router, extract::{Query, Path, State}, Json, http::StatusCode};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use std::collections::HashMap;
use sqlx::PgPool;
use models::puzzle::Puzzle;
use uuid::Uuid;
use dotenv::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Load .env in development (no-op if .env not present)
    dotenv().ok();

    // Use conventional DATABASE_URL; fail fast with a clear message if missing
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set in environment or .env"))?;
    let pool = db::init_pool(&database_url).await?;

    // Import samples
    if args.get(1).map(|s| s.as_str()) == Some("import-samples") {
        println!("Importing sample boards...");
        importer::import_sample_boards(&pool).await?;
        println!("Import complete");
        return Ok(());
    }

    // Server mode
    if args.get(1).map(|s| s.as_str()) == Some("serve") {
        // For dev: permissive CORS (allow all origins/methods). Tighten in production.
        let _cors = CorsLayer::permissive();

        let shared_pool = Arc::new(pool);

        let app = Router::new()
            .route("/puzzles", get(get_puzzles))
            .route("/puzzles/{id}", get(get_puzzle))
            // .layer(cors)
            .with_state(shared_pool.clone());

        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
        println!("Listening on http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        return Ok(());
    }

    // CLI mode (existing functionality)
    if args.len() < 3 {
        eprintln!("Usage: {} <variant> <sample> | serve", args.get(0).unwrap_or(&"program".to_string()));
        std::process::exit(1);
    }

    let game: Game = local_board_importer::game_from_sample_board(&args[1], &args[2]);
    println!();
    let game_status = game.validate(true);
    println!("{}", game.render_game_to_terminal(game_status, true, false));

    Ok(())
}

async fn get_puzzles(
    State(pool): State<Arc<PgPool>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Puzzle>>, (StatusCode, String)> {
    let limit = params.get("limit").and_then(|s| s.parse::<i64>().ok()).unwrap_or(50);
    let pool_ref: &PgPool = pool.as_ref();
    match Puzzle::list(pool_ref, limit).await {
        Ok(puzzles) => Ok(Json(puzzles)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_puzzle(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Puzzle>, (StatusCode, String)> {
    let pool_ref: &PgPool = pool.as_ref();
    match Puzzle::fetch_by_id(pool_ref, id).await {
        Ok(puzzle) => Ok(Json(puzzle)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}
