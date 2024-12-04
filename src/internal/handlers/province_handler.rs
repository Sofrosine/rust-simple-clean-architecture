use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::Response;
use crate::internal::app::usecases::province_usecase::{ProvinceUseCase, ProvinceUseCaseImpl};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

#[derive(Clone)]
pub struct ProvinceHandlerImpl {
    service: ProvinceUseCaseImpl,
}

impl ProvinceHandlerImpl {
    pub fn new(service: ProvinceUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn province_handler_list(handler: web::Data<ProvinceHandlerImpl>) -> impl Responder {
    match handler.service.list().await {
        Ok(provinces) => {
            let response = Response {
                data: provinces,
            };
            HttpResponse::Ok().json(json!({
            "data": response,
            "message": "Successfully fetched provinces",
            "code": 200
        }))
        }
        Err(err) => err.error_response(),
    }
}

pub async fn province_handler_create(
    handler: web::Data<ProvinceHandlerImpl>,
) -> impl Responder {
    match handler.service.create().await {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "Province sync successfully",
            "code": 201
        })),
        Err(err) => err.error_response()
    }
}