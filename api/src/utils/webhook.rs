use std::{
    env,
    io::{Error, ErrorKind},
};

use super::constants::DEFAULT_RESPONSE;
use dotenv::dotenv;
use log::info;
use serenity::all::{CreateAttachment, CreateEmbed, ExecuteWebhook, Http, Webhook};

fn get_webhook_url() -> String {
    dotenv().ok();
    env::var("WEBHOOK_URL").unwrap()
}

pub async fn send_image_embed(url: &str) -> Result<String, Error> {
    let http: Http = Http::new("token");
    let webhook_url = get_webhook_url();
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();
    let embed = CreateEmbed::new().title("From Rust").image(url);
    let builder = ExecuteWebhook::new()
        .content("test")
        .username("serenity")
        .embed(embed);

    let result = match webhook.execute(&http, true, builder).await {
        Ok(Some(message)) => message
            .embeds
            .get(0)
            .and_then(|embed| {
                embed
                    .image
                    .as_ref()
                    .map(|image| image.proxy_url.as_ref().map_or("", |url| url))
            })
            .unwrap_or(DEFAULT_RESPONSE)
            .to_string(),
        _ => DEFAULT_RESPONSE.to_string(),
    };

    Ok(result)
}

pub async fn upload_image_file(path: String) -> Result<String, Error> {
    info!("Uploading file with path = {path}");
    let http: Http = Http::new("token");
    let webhook_url = get_webhook_url();
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();
    match CreateAttachment::path(&path).await {
        Ok(f) => {
            let builder = ExecuteWebhook::new()
                .content("test")
                .add_file(f);

            let result = match webhook.execute(&http, true, builder).await {
                Ok(Some(message)) => message.attachments.get(0).unwrap().proxy_url.clone(),
                _ => DEFAULT_RESPONSE.to_string(),
            };
            Ok(result)
        }
        Err(err) => {
            info!("File could not be found, {err}");
            Err(Error::new(
                ErrorKind::NotFound,
                "File Could not be found".to_string(),
            ))
        }
    }
}
