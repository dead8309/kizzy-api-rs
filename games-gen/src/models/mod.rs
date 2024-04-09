use serde::{ Deserialize, Serialize };

#[derive(Serialize, Debug)]
pub struct Game {
    pub label: String,
    pub title: String,
    pub link: String,
}

#[derive(Deserialize, Serialize)]
pub struct Endpoint {
    pub url: String,
    pub clients: Option<Vec<String>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "US")]
    pub us: String,
    #[serde(rename = "EU")]
    pub eu: String,
    #[serde(rename = "JP")]
    pub jp: String,
    #[serde(rename = "US TITLE")]
    pub us_title: String,
    #[serde(rename = "EU TITLE")]
    pub eu_title: String,
    #[serde(rename = "JP TITLE")]
    pub jp_title: String,
}
