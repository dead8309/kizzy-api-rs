use futures::future::join_all;
use models::Game;
use std::fs;

mod config;
mod fetch;
mod models;
mod utils;
mod xbox;

const OUTPUT_DIR: &str = "../gen/games.json";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = config::get_endpoints();
    let futures = endpoints
        .iter()
        .map(|endpoint| fetch::get_games_from_endpoint(endpoint));
    let tasks = join_all(futures)
        .await
        .into_iter()
        .collect::<Vec<Result<Vec<Game>, reqwest::Error>>>();
    let games = tasks
        .iter()
        .flat_map(|task| task.as_ref().unwrap())
        .collect::<Vec<&Game>>();
    let writer = fs::File::create(OUTPUT_DIR)?;
    serde_json::to_writer(writer, &games)?;
    println!("✅ games.json updated successfully!");
    Ok(())
}
