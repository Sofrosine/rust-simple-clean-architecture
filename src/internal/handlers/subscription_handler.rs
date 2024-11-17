use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use crate::internal::app::usecase::subscription_usecase::{SubscriptionUseCase, SubscriptionUseCaseImpl};


pub trait SubscriptionHandler {
    fn new(service: Arc<SubscriptionUseCaseImpl>) -> Self;
    async fn list(&self) -> impl Responder;
}
pub struct SubscriptionHandlerImpl {
    service: Arc<SubscriptionUseCaseImpl>,
}

impl SubscriptionHandler for SubscriptionHandlerImpl {
    fn new(service: Arc<SubscriptionUseCaseImpl>) -> Self {
        Self { service }
    }

    async fn list(&self) -> impl Responder {
        match self.service.list().await {
            Ok(subscriptions) => HttpResponse::Ok().json(json!({
                "data": subscriptions,
                "message": "Successfully fetched subscriptions",
                "code": 200
            })),
            Err(err) => HttpResponse::InternalServerError().json(json!({
                "message": err.to_string(),
                "code": 500
            }))
        }
    }
}