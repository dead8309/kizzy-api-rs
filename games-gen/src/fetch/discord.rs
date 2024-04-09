use reqwest::Error;
use serde::{ Deserialize, Serialize };

struct DiscordAssetsConfig {
    asset_size: i16,
    asset_format: String,
    assets_url: String,
}

impl DiscordAssetsConfig {
    fn new(application_id: &str) -> DiscordAssetsConfig {
        DiscordAssetsConfig {
            asset_size: 512,
            asset_format: "png".to_string(),
            assets_url: format!("https://discordapp.com/api/oauth2/applications/{}/assets", application_id),
        }
    }
}

#[derive(Deserialize)]
pub struct Asset {
    pub image: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
struct DiscordAsset {
    id: String,
    name: String,
}

pub async fn fetch_discord_assets(clients: Vec<String>) -> Result<Vec<Asset>, Error> {
    let mut results: Vec<Asset> = vec![];
    for client in clients {
        let config = DiscordAssetsConfig::new(&client);
        let data = reqwest::get(config.assets_url).await?.json::<Vec<DiscordAsset>>().await?;

        for asset in data {
            results.push(Asset {
                image: format!(
                    "https://cdn.discordapp.com/app-assets/{}/{}.{}?size={}",
                    client,
                    asset.id,
                    config.asset_format,
                    config.asset_size
                ),
                id: asset.name,
            });
        }
    }
    Ok(results)
}
