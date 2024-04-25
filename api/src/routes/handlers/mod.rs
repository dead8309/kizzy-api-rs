use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use slicestring::Slice;

use crate::utils::{
    api_response::{self, MediaResponse},
    constants::{DEFAULT_RESPONSE, UPLOAD_DIR},
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
                    api_response::ApiResponse::new(200, MediaResponse::new(data))
                }
                _ => api_response::ApiResponse::new(
                    200,
                    MediaResponse::new(DEFAULT_RESPONSE.to_string()),
                ),
            }
        }
        None => {
            api_response::ApiResponse::new(200, MediaResponse::new(DEFAULT_RESPONSE.to_string()))
        }
    }
}

#[derive(MultipartForm)]
struct ImageUpload {
    temp: TempFile,
}

#[post("/upload")]
async fn upload_image(MultipartForm(form): MultipartForm<ImageUpload>) -> impl Responder {
    /* match form.temp.size {
        0 => {
            return api_response::ApiResponse::new(
                400,
                MediaResponse::new(DEFAULT_RESPONSE.to_string()),
            )
        }
        length if length > MAX_FILE_SIZE => {
            return api_response::ApiResponse::new(
                400,
                MediaResponse::new(DEFAULT_RESPONSE.to_string()),
            );
        }
        _ => {}
    };
    */
    let path = format!("{}/{}", UPLOAD_DIR, form.temp.file_name.unwrap());
    let _ = form.temp.file.persist(path.clone());
    match upload_image_file(path).await {
        Ok(asset) => {
            let data = format!(
                "mp:{}",
                asset.slice(asset.find("attachments").unwrap_or(0)..)
            );
            return api_response::ApiResponse::new(200, MediaResponse::new(data));
        }
        Err(err) => {
            log::error!("Error occurred while uploading file, {}", err);
            return api_response::ApiResponse::new(
                200,
                MediaResponse::new(DEFAULT_RESPONSE.to_string()),
            );
        }
    }
}
