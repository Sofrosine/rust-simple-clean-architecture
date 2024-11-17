use sqlx::{query_as, Error, PgPool};
use crate::internal::entities::subscription::Subscription;

pub trait SubscriptionRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self) -> Result<Vec<Subscription>, Error>;
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

    async fn list(&self) -> Result<Vec<Subscription>, Error> {
        let query = r#"
            SELECT * FROM subscriptions ORDER BY price DESC
        "#;

        let rows = query_as(query)
            .fetch_all(&self.database)
            .await?;

        Ok(rows)
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
