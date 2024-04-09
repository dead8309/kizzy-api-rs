use reqwest::Error;
use serde::Deserialize;

use crate::models::Game;

#[derive(Deserialize)]
struct XboxRawFileResponse {
    #[serde(rename = "Games List")]
    games_list: Vec<GamesList>
}

#[derive(Deserialize)]
struct GamesList {
    #[serde(rename = "Games")]
    games: Vec<XboxGames>
}

#[derive(Deserialize)]
struct XboxGames {
    #[serde(rename = "titlename")]
    title_name: String,
    #[serde(rename = "titleimage")]
    _title_image: String,
    #[serde(rename = "titleicon")]
    title_icon: Option<String>,
    #[serde(rename = "titlebackground")]
    _title_background: Option<String>,
    #[serde(rename = "type")]
    r#_type: Option<i32>
}

pub async fn fetch_xbox_games(url: String) -> Result<Vec<Game>, Error> {
    let data = reqwest::get(url)
        .await?
        .json::<XboxRawFileResponse>()
        .await?;
    let result = data.games_list[0].games.iter().map(|game| {
        Game {
            label: "xbox".to_string(),
            title: game.title_name.clone(),
            link: game.title_icon.clone().unwrap_or("".to_string())
        }
    }).collect();
    Ok(result)
}