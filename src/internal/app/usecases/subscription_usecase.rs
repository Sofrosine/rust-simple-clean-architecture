use crate::internal::app::repositories::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};
use crate::internal::entities::subscription::Subscription;
use crate::pkg::dto::subscription_dto::{CreateSubscriptionDto, UpdateSubscriptionDto};
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use std::fmt::Debug;
use crate::internal::app::repositories::subscription_type_repository::{SubscriptionTypeRepository, SubscriptionTypeRepositoryImpl};

pub trait SubscriptionUseCase {
    fn new(repository: SubscriptionRepositoryImpl, subscription_type_repository: SubscriptionTypeRepositoryImpl) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateSubscriptionDto>) -> Result<(), ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateSubscriptionDto>) -> Result<(), ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct SubscriptionUseCaseImpl {
    repository: SubscriptionRepositoryImpl,
    subscription_type_repository: SubscriptionTypeRepositoryImpl,
}

impl SubscriptionUseCase for SubscriptionUseCaseImpl {
    fn new(repository: SubscriptionRepositoryImpl, subscription_type_repository: SubscriptionTypeRepositoryImpl) -> Self {
        Self { repository, subscription_type_repository }
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
        let CreateSubscriptionDto { name, price, subscription_type_id } = form.into_inner();

        // Validate input
        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription name".to_string()),
                Some("FAILED".to_string()),
            ));
        }
        if price <= 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription price".to_string()),
                Some("FAILED".to_string()),
            ));
        }
        if subscription_type_id.to_string().trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription type id".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        // Check if subscription_type_id exists
        match self.subscription_type_repository.get_by_id(subscription_type_id).await {
            Ok(_) => { /* subscription_type_id exists, proceed */ },
            Err(sqlx::Error::RowNotFound) => {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some("Subscription type ID not found".to_string()),
                    Some("FAILED".to_string()),
                ));
            }
            Err(error) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(format!("Failed to fetch subscription type: {}", error)),
                    Some("FAILED".to_string()),
                ));
            }
        }

        // Create subscription
        let subscription = Subscription {
            id: uuid::Uuid::new_v4(),
            name,
            price,
            subscription_type_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&subscription).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn update(&self, id: String, form: Json<UpdateSubscriptionDto>) -> Result<(), ErrorResponse> {
        let UpdateSubscriptionDto { name, price , subscription_type_id} = form.into_inner();

        let subscription = match self.repository.get_by_id(id.parse().unwrap()).await {
            Ok(subscription) => subscription,
            Err(error) => return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        };

        let updated_name = name.unwrap_or(subscription.name);
        let updated_price = price.unwrap_or(subscription.price);
        let updated_subscription_type_id = subscription_type_id.unwrap_or(subscription.subscription_type_id);

        let updated_subscription = Subscription{
            id: subscription.id,
            name: updated_name.trim().to_string(),
            price: updated_price,
            subscription_type_id: updated_subscription_type_id,
            created_at: subscription.created_at,
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.update(&updated_subscription).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        }
    }

    async fn delete(&self, id: String) -> Result<(), ErrorResponse> {
        match self.repository.delete(id.parse().unwrap()).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        }
    }
}