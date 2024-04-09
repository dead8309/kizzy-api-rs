use futures::future::join_all;
use models::Game;
use std::fs;

mod fetch;
mod models;
mod xbox;
mod config;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = config::get_endpoints();
    let futures = endpoints.iter().map(|endpoint| { fetch::get_games_from_endpoint(endpoint) });
    let tasks = join_all(futures).await
        .into_iter()
        .collect::<Vec<Result<Vec<Game>, reqwest::Error>>>();
    let games = tasks
        .iter()
        .flat_map(|task| task.as_ref().unwrap())
        .collect::<Vec<&Game>>();
    let writer = fs::File::create("games.json")?;
    serde_json::to_writer(writer, &games)?;
    Ok(())
}
