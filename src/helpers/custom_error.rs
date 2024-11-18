use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub trait ResponseError {
    fn status_code(&self) -> StatusCode;
    fn error_response(&self) -> HttpResponse<BoxBody>;
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub message: Option<String>,
    pub status: Option<String>,
    pub err_type: StatusCode
}

impl ErrorResponse {
    pub fn new(err_type: StatusCode,
               message: Option<String>,
               status: Option<String>,
    ) -> ErrorResponse {
        ErrorResponse {err_type, message, status}
    }
}

#[derive(Serialize, Deserialize)]
struct ActualResponse {
    pub message: Option<String>,
    pub status: Option<String>,
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.err_type
    }

    fn error_response(&self) -> HttpResponse {
        let res = ActualResponse {
            message: self.message.clone(),
            status: self.status.clone()
        };
        HttpResponse::build(self.status_code()).json(res)
    }
}