use std::fmt::Debug;
use std::sync::Arc;
use sqlx::Error;
use crate::internal::app::repository::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};
use crate::internal::entity::subscription::Subscription;


pub trait SubscriptionUseCase {
    fn new(repository: Arc<SubscriptionRepositoryImpl>) -> Self;
    async fn list(&self) -> Result<Vec<Subscription>, Error>;
}

#[derive(Debug)]
pub struct SubscriptionUseCaseImpl {
    repository: Arc<SubscriptionRepositoryImpl>,
}

impl SubscriptionUseCase for SubscriptionUseCaseImpl {
    fn new(repository: Arc<SubscriptionRepositoryImpl>) -> Self {
        Self { repository }
    }

    async fn list(&self) -> Result<Vec<Subscription>, Error> {
        match self.repository.list().await {
            Ok(subscriptions) => Ok(subscriptions),
            Err(error) => Err(error),
        }
    }
}