use crate::internal::app::repositories::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};
use crate::internal::entities::subscription::Subscription;
use crate::pkg::dto::subscription_dto::CreateSubscriptionDto;
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use std::fmt::Debug;

pub trait SubscriptionUseCase {
    fn new(repository: SubscriptionRepositoryImpl) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateSubscriptionDto>) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct SubscriptionUseCaseImpl {
    repository: SubscriptionRepositoryImpl,
}

impl SubscriptionUseCase for SubscriptionUseCaseImpl {
    fn new(repository: SubscriptionRepositoryImpl) -> Self {
        Self { repository }
    }

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), ErrorResponse> {
        if page == 0 || page_size == 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid pagination parameters".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let offset = (page - 1) * page_size;

        match self.repository.list(offset, page_size).await {
            Ok((subscriptions, total_data)) => Ok((subscriptions, total_data)),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string())
            )),
        }
    }

    async fn create(&self, form: Json<CreateSubscriptionDto>) -> Result<(), ErrorResponse> {
        let CreateSubscriptionDto { name, price } = form.into_inner();
        // Validate input
        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription name".to_string()),
                Some("FAILED".to_string())
            ));
        }
        if price <= 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription price".to_string()),
                Some("FAILED".to_string())
            ));
        }

        let subscription = Subscription {
            id: uuid::Uuid::new_v4(),
            name,
            price,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&subscription).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string())
            )),
        }
    }
}