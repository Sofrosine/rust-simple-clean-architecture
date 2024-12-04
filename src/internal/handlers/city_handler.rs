use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::Response;
use crate::internal::app::usecases::city_usecase::{CityUseCase, CityUseCaseImpl};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

#[derive(Clone)]
pub struct CityHandlerImpl {
    service: CityUseCaseImpl,
}

impl CityHandlerImpl {
    pub fn new(service: CityUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn city_handler_list(handler: web::Data<CityHandlerImpl>) -> impl Responder {
    match handler.service.list().await {
        Ok(cities) => {
            let response = Response {
                data: cities,
            };
            HttpResponse::Ok().json(json!({
            "data": response,
            "message": "Successfully fetched cities",
            "code": 200
        }))
        }
        Err(err) => err.error_response(),
    }
}

pub async fn city_handler_create(
    handler: web::Data<CityHandlerImpl>,
) -> impl Responder {
    match handler.service.create().await {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "City sync successfully",
            "code": 201
        })),
        Err(err) => err.error_response()
    }
}