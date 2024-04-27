use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Contributor {
    name: String,
    url: String,
    avatar: String,
}

#[derive(Deserialize)]
struct GithubResponse {
    login: String,
    avatar_url: String,
    html_url: String,
}

pub async fn get_gitub_contributors() -> Result<Vec<Contributor>, Error> {
    let client = reqwest::Client::builder()
        .user_agent("RustActix")
        .build().unwrap();
    let response = client.get("https://api.github.com/repos/dead8309/Kizzy/contributors")
        .send()
        .await?
        .json::<Vec<GithubResponse>>()
        .await?;
    let contributors = response
        .iter()
        .map(|contributor| Contributor {
            name: contributor.login.to_owned(),
            url: contributor.html_url.to_owned(),
            avatar: contributor.avatar_url.to_owned(),
        })
        .collect::<Vec<Contributor>>();

    Ok(contributors)
}
