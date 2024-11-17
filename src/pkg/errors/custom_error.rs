use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub trait CustomResponse {
    fn status_code(&self) -> StatusCode;
    fn response(&self) -> HttpResponse<BoxBody>;
}

#[derive(Debug)]
pub struct SResponse {
    pub message: Option<String>,
    pub status: Option<String>,
    pub status_code: StatusCode
}

impl SResponse {
    pub fn new(status_code: StatusCode,
               message: Option<String>,
               status: Option<String>,
    ) -> SResponse {
        SResponse {status_code, message, status}
    }
}

#[derive(Serialize, Deserialize)]
struct ActualResponse {
    pub message: Option<String>,
    pub status: Option<String>,
}

impl CustomResponse for SResponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn response(&self) -> HttpResponse {
        let res = ActualResponse {
            message: self.message.clone(),
            status: self.status.clone()
        };
        HttpResponse::build(self.status_code()).json(res)
    }
}