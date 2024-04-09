pub mod discord;

use reqwest::Error;

use crate::{models::{Endpoint, Game, Record}, utils, xbox};

use self::discord::fetch_discord_assets;

pub async fn get_games_from_endpoint(endpoint: &Endpoint) -> Result<Vec<Game>, Error> {
    let mut games: Vec<Game> = vec![];
    match &endpoint.clients {
        Some(_clients) => {
            let data = reqwest::get(endpoint.url.clone()).await?.text().await?;
            let records = utils::parse_csv(data);
            games.extend(convert_to_game(endpoint, records).await?);
        }
        None => {
            // Since we dont have a client, we can assume that we are fetching from xbox
            games.extend(xbox::fetch_xbox_games(endpoint.url.clone()).await?);
        }
    }
    Ok(games)
}


async fn convert_to_game(endpoint: &Endpoint, records: Vec<Record>) -> Result<Vec<Game>, Error> {
    let assets = fetch_discord_assets(endpoint.clients.clone().unwrap_or_default()).await?;
    let games: Vec<Game> = records
        .iter()
        .map(|record| {
            let binding = vec![
                record.us_title.clone(),
                record.eu_title.clone(),
                record.jp_title.clone()
            ];
            let title = binding
                .iter()
                .filter(|v| !v.is_empty())
                .next()
                .unwrap();
            let link = assets
                .iter()
                .find(|asset| {
                    let len = asset.id.len();
                    asset.id[0..len - 3].to_string() == record.id
                })
                .map(|asset| asset.image.clone())
                .unwrap_or_default();

            Game {
                label: endpoint.name.clone(),
                title: title.clone(),
                link,
            }
        })
        .collect();
    Ok(games)
}