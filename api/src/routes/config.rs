use crate::utils::constants::{MAX_FILE_SIZE, UPLOAD_DIR};
use actix_multipart::{
    form::{tempfile::TempFileConfig, MultipartFormConfig},
    MultipartError,
};
use actix_web::{web, Error, HttpRequest};

use super::handlers::{get_image, upload_image, games, user, contributors};

fn handle_multipart_error(err: MultipartError, _: &HttpRequest) -> Error {
    log::info!("Multipart error: {}", err);

    return actix_web::Error::from(err);
}

pub fn config(config: &mut web::ServiceConfig) {
    let tmp_config = TempFileConfig::default().directory(UPLOAD_DIR);
    let multipart_config = MultipartFormConfig::default()
        .memory_limit(MAX_FILE_SIZE)
        .total_limit(MAX_FILE_SIZE)
        .error_handler(handle_multipart_error);
    config
        .app_data(multipart_config)
        .app_data(tmp_config)
        .service(get_image)
        .service(upload_image)
        .service(games)
        .service(user)
        .service(contributors);
}
