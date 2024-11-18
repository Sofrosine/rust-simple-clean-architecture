use sqlx::{query_as, Error, PgPool};
use crate::internal::entities::subscription::Subscription;

pub trait SubscriptionRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), Error>;
    async fn create(&self, subscription: &Subscription) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct SubscriptionRepositoryImpl {
    database: PgPool,
}

impl SubscriptionRepository for SubscriptionRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), Error> {
        let query = r#"
            SELECT * FROM subscriptions ORDER BY price ASC LIMIT $1 OFFSET $2
        "#;

        let count_query = r#"
            SELECT COUNT(*) AS total FROM subscriptions
        "#;

        let rows = query_as(query)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.database)
            .await?;

        let total: (i64,) = query_as(count_query)
            .fetch_one(&self.database)
            .await?;

        Ok((rows, total.0))
    }

    async fn create(&self, subscription: &Subscription) -> Result<(), Error> {
        let query = r#"
            INSERT INTO subscriptions (id, name, price, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        sqlx::query(query)
            .bind(subscription.id)
            .bind(&subscription.name)
            .bind(subscription.price)
            .bind(subscription.created_at)
            .bind(subscription.updated_at)
            .bind(subscription.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }
}
