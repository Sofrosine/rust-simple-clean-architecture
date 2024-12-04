use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;
use crate::internal::entities::subscription::Subscription;

pub trait SubscriptionRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<Subscription>, i64), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Subscription, Error>;
    async fn get_by_subscription_type_id(&self, id: Uuid) -> Result<Vec<Subscription>, Error>;
    async fn create(&self, subscription: &Subscription) -> Result<(), Error>;
    async fn update(&self, subscription: &Subscription) -> Result<(), Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
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

    async fn get_by_id(&self, id: Uuid) -> Result<Subscription, Error> {
        let query = r#"
            SELECT * FROM subscriptions WHERE id = $1 AND deleted_at IS NULL
        "#;

        let subscription = query_as(query).bind(id).fetch_one(&self.database).await?;

        Ok(subscription)
    }

    async fn get_by_subscription_type_id(&self, id: Uuid) -> Result<Vec<Subscription>, Error> {
        let query = r#"
            SELECT * FROM subscriptions WHERE subscription_type_id = $1 AND deleted_at IS NULL
        "#;

        let subscription = query_as(query).bind(id).fetch_all(&self.database).await?;

        Ok(subscription)
    }


    async fn create(&self, subscription: &Subscription) -> Result<(), Error> {
        let query = r#"
            INSERT INTO subscriptions (id, name, price, subscription_type_id, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#;

        sqlx::query(query)
            .bind(subscription.id)
            .bind(&subscription.name)
            .bind(subscription.price)
            .bind(subscription.subscription_type_id)
            .bind(subscription.created_at)
            .bind(subscription.updated_at)
            .bind(subscription.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn update(&self, subscription: &Subscription) -> Result<(), Error> {
        let query = r#"
        UPDATE subscriptions
            SET name = $1, price = $2, updated_at = $3
            WHERE id = $4
        "#;

        sqlx::query(query)
            .bind(&subscription.name)
            .bind(subscription.price)
            .bind(subscription.updated_at)
            .bind(subscription.id)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query = r#"
            DELETE FROM subscriptions WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.database)
            .await?;

        Ok(())
    }
}
