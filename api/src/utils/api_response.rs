use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub struct ApiResponse<T> {
    pub status_code: u16,
    pub body: T,
    response_code: StatusCode,
}

impl<T> ApiResponse<T> {
    pub fn new(status_code: u16, body: T) -> Self {
        let response_code =
            StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        ApiResponse {
            status_code,
            body,
            response_code,
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self.body) {
            Ok(body) => {
                let response = BoxBody::new(web::BytesMut::from(body.as_bytes()));
                HttpResponse::new(self.response_code).set_body(response)
            }
            Err(err) => {
                let error = HttpResponse::InternalServerError()
                    .body(format!("Failed to serialize response body: {}", err));
                error
            }
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct MediaResponse {
    id: String,
}

impl MediaResponse {
    pub fn new(id: String) -> Self {
        MediaResponse { id }
    }
}
