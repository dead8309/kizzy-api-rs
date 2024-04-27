use std::env;

use dotenv::dotenv;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client, Error,
};
use serde::{Deserialize, Serialize};
use serenity::all::Member;

use super::constants::{DISCORD_API, GUILD_ID, MINIMUM_ROLES_REQUIRED_FOR_VERIFIED, SPECIAL_LINK};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: String,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: i32,
    flags: i32,
    banner: Option<String>,
    accent_color: i32,
    global_name: String,
    avatar_decoration_data: Option<String>,
    banner_color: String,
    clan: Option<String>,
    verified: Option<bool>,
    special: Option<String>,
    badges: Option<Vec<Badge>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Badge {
    name: String,
    icon: String,
}

/**
 * TODO: DO BADGES PART
 *
 * Although i dont think we need to send badges now as the image links to the badges
 * will get expired after 30min since this new change in discord policy.
 *
 * [commit](https://github.com/discord/discord-api-docs/commit/4efb710136ed7ac010b2e30642413412937a6d62)
 */
pub async fn get_user_by_id(id: u64) -> Result<User, Error> {
    match get_user(id).await {
        Ok(mut user) => {
            let is_verified = check_guild_membership(id).await.unwrap_or(false);
            user.verified = Some(is_verified);
            user.special = Some(SPECIAL_LINK.to_string());
            user.badges = Some(vec![]);
            Ok(user)
        }
        Err(err) => Err(err),
    }
}

async fn get_user(id: u64) -> Result<User, Error> {
    let (client, token) = get_client_and_token();
    let response = client
        .get(format!("{}/users/{}", DISCORD_API, id))
        .header(AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
        .send()
        .await;

    match response {
        Ok(data) => match data.json::<User>().await {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

async fn check_guild_membership(user_id: u64) -> Result<bool, Error> {
    let (client, token) = get_client_and_token();
    let member = client
        .get(format!(
            "{}/guilds/{}/members/{}",
            DISCORD_API, GUILD_ID, user_id
        ))
        .header(AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
        .send()
        .await?
        .json::<Member>()
        .await?;
    Ok(member.roles.len() >= MINIMUM_ROLES_REQUIRED_FOR_VERIFIED)
}

fn get_client_and_token() -> (Client, String) {
    dotenv().ok();
    let token = format!(
        "Bot {}",
        env::var("BOT_TOKEN").expect("BOT_TOKEN is not provided")
    );
    (reqwest::Client::new(), token)
}
