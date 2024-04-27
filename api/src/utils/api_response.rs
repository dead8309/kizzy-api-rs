use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        let response_code =
            StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        ApiResponse {
            status_code,
            body,
            response_code,
        }
    }
    pub fn from_json<T: Serialize>(status_code: u16, body: T) -> Self {
        match serde_json::to_string(&body) {
            Ok(body) => Self::new(status_code, body),
            Err(err) => {
                log::error!("Error occurred while deserializing object: {}", err);
                Self::new(
                    status_code,
                    "Failed to serialize response body: {}".to_string(),
                )
            }
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let response = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(response)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MediaResponse {
    id: String,
}

impl MediaResponse {
    pub fn new(id: String) -> Self {
        MediaResponse { id }
    }
}
