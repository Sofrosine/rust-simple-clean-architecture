use std::sync::Arc;
use sqlx::{query_as, Error, PgPool};
use crate::internal::entity::subscription::Subscription;

pub trait SubscriptionRepository {
    fn new(database: Arc<PgPool>) -> Self;
    async fn list(&self) -> Result<Vec<Subscription>, Error>;
}

#[derive(Debug)]
pub struct SubscriptionRepositoryImpl {
    database: Arc<PgPool>,
}

impl SubscriptionRepository for SubscriptionRepositoryImpl {
    fn new(database: Arc<PgPool>) -> Self {
        Self { database }
    }

    async fn list(&self) -> Result<Vec<Subscription>, Error> {
        let query = r#"
            SELECT * FROM subscriptions ORDER BY price DESC
        "#;

        let rows = query_as(query)
            .fetch_all(&*self.database)
            .await?;

        Ok(rows)
    }
}
