use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::Response;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::internal::app::usecases::auth_usecase::{AuthUseCase, AuthUseCaseImpl};
use crate::pkg::dto::auth_dto::RegisterDto;

#[derive(Clone)]
pub struct AuthHandlerImpl {
    service: AuthUseCaseImpl,
}

impl AuthHandlerImpl {
    pub fn new(service: AuthUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn register(handler: web::Data<AuthHandlerImpl>,
                      input: web::Json<RegisterDto>,
) -> impl Responder {
    match handler.service.register(input).await {
        Ok(user) => {
            let response = Response {
                data: user,
            };
            HttpResponse::Ok().json(json!({
            "data": response,
            "message": "Successfully created user",
            "code": 200
        }))
        }
        Err(err) => err.error_response(),
    }
}
