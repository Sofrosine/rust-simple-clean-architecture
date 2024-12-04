use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Query;
use serde_json::json;
use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::{PaginatedResponse, PaginationParams};
use crate::internal::app::usecases::subscription_type_usecase::{SubscriptionTypeUseCase, SubscriptionTypeUseCaseImpl};
use crate::pkg::dto::subscription_type_dto::{CreateSubscriptionTypeDto, UpdateSubscriptionTypeDto};

#[derive(Clone)]
pub struct SubscriptionTypeHandlerImpl {
    service: SubscriptionTypeUseCaseImpl,
}

impl SubscriptionTypeHandlerImpl {
    pub fn new(service: SubscriptionTypeUseCaseImpl) -> SubscriptionTypeHandlerImpl {
        Self { service }
    }
}

pub async fn subscription_type_handler_list(handler: web::Data<SubscriptionTypeHandlerImpl>, params: Query<PaginationParams>) -> impl Responder {
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

pub async fn subscription_type_handler_create(
    handler: web::Data<SubscriptionTypeHandlerImpl>,
    input: web::Json<CreateSubscriptionTypeDto>,
) -> impl Responder {
    match handler.service.create(input).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "Subscription created successfully",
            "code": 201
        })),
        Err(err) => err.error_response()
    }
}

pub async fn subscription_type_handler_update(
    handler: web::Data<SubscriptionTypeHandlerImpl>,
    path: web::Path<String>,
    input: web::Json<UpdateSubscriptionTypeDto>,
) -> impl Responder {
    let path_id = path.into_inner();
    match handler.service.update(path_id, input).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Subscription updated successfully",
            "code": 200
        })),
        Err(err) => err.error_response()
    }
}

pub async fn subscription_type_handler_delete(
    handler: web::Data<SubscriptionTypeHandlerImpl>,
    path: web::Path<String>,
) -> impl Responder {
    let path_id = path.into_inner();
    match handler.service.delete(path_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Subscription deleted successfully",
            "code": 200
        })),
        Err(err) => err.error_response()
    }
}