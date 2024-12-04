use crate::internal::app::repositories::subscription_type_repository::{SubscriptionTypeRepository, SubscriptionTypeRepositoryImpl};
use crate::internal::entities::subscription_type::{SubscriptionType, SubscriptionTypeResponse};
use crate::pkg::dto::subscription_type_dto::{CreateSubscriptionTypeDto, UpdateSubscriptionTypeDto};
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use std::fmt::Debug;
use crate::internal::app::repositories::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};

pub trait SubscriptionTypeUseCase {
    fn new(repository: SubscriptionTypeRepositoryImpl, subscription_repository_impl: SubscriptionRepositoryImpl) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<SubscriptionTypeResponse>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateSubscriptionTypeDto>) -> Result<(), ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateSubscriptionTypeDto>) -> Result<(), ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct SubscriptionTypeUseCaseImpl {
    repository: SubscriptionTypeRepositoryImpl,
    subscription_repository: SubscriptionRepositoryImpl,
}

impl SubscriptionTypeUseCase for SubscriptionTypeUseCaseImpl {
    fn new(repository: SubscriptionTypeRepositoryImpl, subscription_repository: SubscriptionRepositoryImpl) -> Self {
        Self { repository, subscription_repository }
    }

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<SubscriptionTypeResponse>, i64), ErrorResponse> {
        if page == 0 || page_size == 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid pagination parameters".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let offset = (page - 1) * page_size;

        let (subscription_types, total_data) = self.repository.list(offset, page_size)
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(err.to_string()),
                    Some("FAILED".to_string()),
                )
            })?;

        let (response_data, ) = tokio::try_join!(
            async {
                let mut responses =  Vec::new();
                for st in subscription_types {
                    let subscriptions = self.subscription_repository.get_by_subscription_type_id(st.id).await.unwrap_or_else(|_| vec![]);
                    responses.push(SubscriptionTypeResponse{
                        id: st.id,
                        name: st.name,
                        created_at: st.created_at,
                        updated_at: st.updated_at,
                        deleted_at: st.deleted_at,
                            subscriptions: subscriptions,
                    });
                }
                Ok(responses)
            }
        )?;
        Ok((response_data, total_data))
    }

    async fn create(&self, form: Json<CreateSubscriptionTypeDto>) -> Result<(), ErrorResponse> {
        let CreateSubscriptionTypeDto { name } = form.into_inner();
        // Validate input
        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid subscription_type name".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let subscription_type = SubscriptionType {
            id: uuid::Uuid::new_v4(),
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&subscription_type).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn update(&self, id: String, form: Json<UpdateSubscriptionTypeDto>) -> Result<(), ErrorResponse> {
        let UpdateSubscriptionTypeDto { name } = form.into_inner();

        let subscription_type = match self.repository.get_by_id(id.parse().unwrap()).await {
            Ok(subscription_type) => subscription_type,
            Err(error) => return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        };

        let updated_name = name.unwrap_or(subscription_type.name);

        let updated_subscription_type = SubscriptionType {
            id: subscription_type.id,
            name: updated_name.trim().to_string(),
            created_at: subscription_type.created_at,
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.update(&updated_subscription_type).await {
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