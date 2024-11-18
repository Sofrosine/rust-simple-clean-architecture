use crate::internal::app::usecases::subscription_usecase::{SubscriptionUseCase, SubscriptionUseCaseImpl};
use crate::pkg::dto::subscription_dto::CreateSubscriptionDto;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Query;
use serde_json::json;
use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::{PaginatedResponse, PaginationParams};

#[derive(Clone)]
pub struct SubscriptionHandlerImpl {
    service: SubscriptionUseCaseImpl,
}

impl SubscriptionHandlerImpl {
    pub fn new(service: SubscriptionUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn subscription_handler_list(handler: web::Data<SubscriptionHandlerImpl>, params: Query<PaginationParams>) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match handler.service.list(page, page_size).await {
        Ok((subscriptions, total_data)) => {
            let total_pages = (total_data as f32 / page_size as f32).ceil() as u32;
            let response = PaginatedResponse {
                data: subscriptions,
                page_size,
                page,
                total_pages,
                total_data: total_data as u32,
            };
            HttpResponse::Ok().json(json!({
            "data": response,
            "message": "Successfully fetched subscriptions",
            "code": 200
        }))
        }
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