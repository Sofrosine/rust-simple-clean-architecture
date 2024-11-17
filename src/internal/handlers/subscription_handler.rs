use crate::internal::app::usecases::subscription_usecase::{SubscriptionUseCase, SubscriptionUseCaseImpl};
use crate::pkg::dto::subscription_dto::CreateSubscriptionDto;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::pkg::errors::custom_error::ResponseError;

#[derive(Clone)]
pub struct SubscriptionHandlerImpl {
    service: SubscriptionUseCaseImpl,
}

impl SubscriptionHandlerImpl {
    pub fn new(service: SubscriptionUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn subscription_handler_list(handler: web::Data<SubscriptionHandlerImpl>) -> impl Responder {
    match handler.service.list().await {
        Ok(subscriptions) => HttpResponse::Ok().json(json!({
            "data": subscriptions,
            "message": "Successfully fetched subscriptions",
            "code": 200
        })),
        Err(err) => err.error_response(),
    }
}

pub async fn subscription_handler_create(
    handler: web::Data<SubscriptionHandlerImpl>,
    input: web::Json<CreateSubscriptionDto>,
) -> impl Responder {
    match handler.service.create(input).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "Subscription created successfully",
            "code": 201
        })),
        Err(err) => err.error_response()
    }
}