use crate::variants::{self, Variant, VariantIdentifier};
use crate::game::Game;
use anyhow::Context;
use sqlx::{PgPool, Row};
use std::path::PathBuf;
use std::fs;
use serde_json::json;
use chrono::Utc;

fn map_difficulty(s: &str) -> Option<i16> {
    let s_trim = s.trim().to_lowercase();
    if s_trim.is_empty() { return None; }
    if let Ok(n) = s_trim.parse::<i16>() { return Some(n); }
    match s_trim.as_str() {
        "easy" => Some(1),
        "medium" => Some(2),
        "hard" => Some(3),
        "very hard" => Some(4),
        _ => None,
    }
}

pub async fn import_sample_boards(pool: &PgPool) -> anyhow::Result<()> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("src");
    base.push("sample_boards");

    let mut inserted = 0usize;
    for variant_dir in fs::read_dir(&base).context("reading sample_boards dir")? {
        let variant_dir = variant_dir?.path();
        if !variant_dir.is_dir() { continue; }
        for entry in fs::read_dir(&variant_dir).context("reading variant dir")? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("csv") { continue; }

            // parse the CSV manually to extract metadata and game
            let mut rdr = csv::ReaderBuilder::new()
                .comment(Some(b'#'))
                .has_headers(false)
                .flexible(true)
                .from_path(&path)
                .map_err(|e| anyhow::anyhow!("opening sample file {}: {}", path.display(), e))?;

            let mut records = rdr.records().enumerate();
            let title = records.next().unwrap().1?.get(0).unwrap_or("").to_string();
            let author = records.next().unwrap().1?.get(0).unwrap_or("").to_string();
            let difficulty_raw = records.next().unwrap().1?.get(0).unwrap_or("").to_string();
            let variants_record = records.next().unwrap().1?;
            let variants_strings: Vec<&str> = variants_record.iter().collect();
            let mut variants_used: Vec<Variant> = Vec::new();
            for vstr in variants_strings {
                let mut found = false;
                for av in Variant::ALLOWED_VARIANTS {
                    if vstr.trim().eq_ignore_ascii_case(av.alias) {
                        variants_used.push(av.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    // ignore unknown variants but warn
                    eprintln!("warning: unknown variant '{}' in {}", vstr, path.display());
                }
            }
            if !variants_used.iter().any(|v| v.identifier == VariantIdentifier::STANDARD) {
                variants_used.push(Variant::from_alias("standard").unwrap().clone());
            }

            // Build Game by applying additional parsing per variant
            let board: [[char; 9]; 9] = [[' '; 9]; 9];
            let mut game = Game { board, variants_used: variants_used.clone() };
            let variants_snapshot = game.variants_used.clone();
            for v in &variants_snapshot {
                variants::apply_additional_parsing(v, &mut records, &mut game)
                    .map_err(|e| anyhow::anyhow!("parsing board for {}: {}", path.display(), e))?;
            }

            // Convert board to Vec<Vec<u8>> with 0 for empty
            let board_vec: Vec<Vec<u8>> = game.board.iter()
                .map(|row| row.iter().map(|c| if *c == Game::EMPTY_DIGIT { 0 } else { c.to_digit(10).unwrap_or(0) as u8 }).collect())
                .collect();

            // determine variant string (first alias)
            let variant_alias = game.variants_used.get(0).map(|v| v.alias.to_string()).unwrap_or_else(|| "standard".to_string());

            let difficulty = map_difficulty(&difficulty_raw);

            let variant_data = json!({
                "source_file": path.to_string_lossy().to_string(),
                "raw_difficulty": difficulty_raw,
                "variants": game.variants_used.iter().map(|v| v.alias).collect::<Vec<_>>()
            });

            // skip if name already exists
            let existing: Option<(uuid::Uuid,)> = sqlx::query_as("SELECT id FROM puzzles WHERE name = $1 LIMIT 1")
                .bind(&title)
                .fetch_optional(pool)
                .await?;
            if existing.is_some() {
                println!("Skipping existing puzzle '{}'", title);
                continue;
            }

            // insert
            let res = sqlx::query("INSERT INTO puzzles (name, author, variant, difficulty, board, variant_data, created_at, updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id")
                .bind(&title)
                .bind(if author.is_empty() { Option::<String>::None } else { Some(author.clone()) })
                .bind(&variant_alias)
                .bind(difficulty)
                .bind(serde_json::to_value(&board_vec)?)
                .bind(variant_data)
                .bind(Utc::now())
                .bind(Utc::now())
                .fetch_one(pool)
                .await?;

            let inserted_id: uuid::Uuid = res.try_get("id")?;
            println!("Inserted puzzle '{}' id={}", title, inserted_id);
            inserted += 1;
        }
    }

    println!("Imported {} puzzles", inserted);
    Ok(())
}
