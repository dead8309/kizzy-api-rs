use std::{
    fs::File,
    io::{BufReader, Read},
};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Responder};
use log::error;
use serde::Deserialize;
use slicestring::Slice;

use crate::utils::{
    api_response::{self, MediaResponse},
    constants::{DEFAULT_RESPONSE, UPLOAD_DIR},
    discord, github,
    webhook::{send_image_embed, upload_image_file},
};

#[derive(Deserialize)]
struct ImageQuery {
    url: Option<String>,
}

#[get("/image")]
async fn get_image(req: web::Query<ImageQuery>) -> impl Responder {
    match req.url.clone() {
        Some(url) => {
            let result = send_image_embed(&url).await;
            match result {
                Ok(asset) => {
                    let mut data = if asset.starts_with("https://media.discordapp.net/") {
                        asset.slice(asset.find("attachments").unwrap_or(0)..)
                    } else {
                        asset.slice(asset.find("external").unwrap_or(0)..)
                    };
                    data = format!("mp:{}", data);
                    api_response::ApiResponse::from_json(200, MediaResponse::new(data))
                }
                _ => api_response::ApiResponse::from_json(
                    200,
                    MediaResponse::new(DEFAULT_RESPONSE.to_string()),
                ),
            }
        }
        None => api_response::ApiResponse::from_json(
            200,
            MediaResponse::new(DEFAULT_RESPONSE.to_string()),
        ),
    }
}

#[derive(MultipartForm)]
struct ImageUpload {
    temp: TempFile,
}

#[post("/upload")]
async fn upload_image(MultipartForm(form): MultipartForm<ImageUpload>) -> impl Responder {
    let path = format!("{}/{}", UPLOAD_DIR, form.temp.file_name.unwrap());
    let _ = form.temp.file.persist(path.clone());
    match upload_image_file(path).await {
        Ok(asset) => {
            let data = format!(
                "mp:{}",
                asset.slice(asset.find("attachments").unwrap_or(0)..)
            );
            return api_response::ApiResponse::from_json(200, MediaResponse::new(data));
        }
        Err(err) => {
            log::error!("Error occurred while uploading file, {}", err);
            return api_response::ApiResponse::from_json(
                200,
                MediaResponse::new(DEFAULT_RESPONSE.to_string()),
            );
        }
    }
}

#[get("/games")]
async fn games() -> impl Responder {
    let reader = File::open("../gen/games.json");
    match reader {
        Ok(file) => {
            let mut contents = String::new();
            let mut reader = BufReader::new(file);
            let _ = reader.read_to_string(&mut contents);
            api_response::ApiResponse::new(200, contents)
        }
        Err(err) => {
            error!("File could not be opened, {}", err);
            api_response::ApiResponse::new(500, "Internal Server Error".to_string())
        }
    }
}

#[get("/user/{userid}")]
async fn user(path: web::Path<u64>) -> impl Responder {
    let user_id = path.into_inner();
    match discord::get_user_by_id(user_id).await {
        Ok(user) => api_response::ApiResponse::from_json(200, user),
        Err(err) => api_response::ApiResponse::new(500, err.to_string()),
    }
}

#[get("/contributors")]
async fn contributors() -> impl Responder {
    let response = github::get_gitub_contributors().await;
    match response {
        Ok(contributors) => api_response::ApiResponse::from_json(200, contributors),
        Err(err) => {
            error!("Error occurred while fetching contributors, {}", err);
            api_response::ApiResponse::new(500, "Internal Server Error".to_string())
        }
    }
}
