use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;
use crate::internal::entities::subscription_type::SubscriptionType;

pub trait SubscriptionTypeRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<SubscriptionType>, i64), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<SubscriptionType, Error>;
    async fn create(&self, subscription_type: &SubscriptionType) -> Result<(), Error>;
    async fn update(&self, subscription_type: &SubscriptionType) -> Result<(), Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct SubscriptionTypeRepositoryImpl {
    database: PgPool,
}

impl SubscriptionTypeRepository for SubscriptionTypeRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<SubscriptionType>, i64), Error> {
        let query = r#"
            SELECT * FROM subscription_types WHERE deleted_at IS NULL ORDER BY name ASC LIMIT $1 OFFSET $2
        "#;

        let count_query = r#"
            SELECT COUNT(*) AS total FROM subscription_types WHERE deleted_at IS NULL
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

    async fn get_by_id(&self, id: Uuid) -> Result<SubscriptionType, Error> {
        let query = r#"
            SELECT * FROM subscription_types WHERE id = $1 AND deleted_at IS NULL
        "#;

        let subscription_type = query_as(query).bind(id).fetch_one(&self.database).await?;

        Ok(subscription_type)
    }

    async fn create(&self, subscription_type: &SubscriptionType) -> Result<(), Error> {
        let query = r#"
            INSERT INTO subscription_types (id, name, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        sqlx::query(query)
            .bind(subscription_type.id)
            .bind(&subscription_type.name)
            .bind(subscription_type.created_at)
            .bind(subscription_type.updated_at)
            .bind(subscription_type.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn update(&self, subscription_type: &SubscriptionType) -> Result<(), Error> {
        let query = r#"
        UPDATE subscription_types
            SET name = $1, updated_at = $3
            WHERE id = $4
        "#;

        sqlx::query(query)
            .bind(&subscription_type.name)
            .bind(subscription_type.updated_at)
            .bind(subscription_type.id)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query = r#"
            DELETE FROM subscription_types WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.database)
            .await?;

        Ok(())
    }
}