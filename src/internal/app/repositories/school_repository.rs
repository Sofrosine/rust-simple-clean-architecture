use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;
use crate::internal::entities::school::School;

pub trait SchoolRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<School>, i64), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<School, Error>;
    async fn get_by_subscription_id(&self, id: Uuid) -> Result<Vec<School>, Error>;
    async fn create(&self, subscription: &School) -> Result<(), Error>;
    async fn update(&self, subscription: &School) -> Result<(), Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct SchoolRepositoryImpl {
    database: PgPool,
}

impl SchoolRepository for SchoolRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<School>, i64), Error> {
        let query = r#"
            SELECT * FROM schools ORDER BY created_at ASC LIMIT $1 OFFSET $2
        "#;

        let count_query = r#"
            SELECT COUNT(*) AS total FROM schools
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

    async fn get_by_id(&self, id: Uuid) -> Result<School, Error> {
        let query = r#"
            SELECT * FROM schools WHERE id = $1 AND deleted_at IS NULL
        "#;

        let subscription = query_as(query).bind(id).fetch_one(&self.database).await?;

        Ok(subscription)
    }

    async fn get_by_subscription_id(&self, id: Uuid) -> Result<Vec<School>, Error> {
        let query = r#"
            SELECT * FROM schools WHERE subscription_id = $1 AND deleted_at IS NULL
        "#;

        let subscription = query_as(query).bind(id).fetch_all(&self.database).await?;

        Ok(subscription)
    }


    async fn create(&self, school: &School) -> Result<(), Error> {
        let query = r#"
            INSERT INTO schools (id, name, address, logo_path, subscription_id, province_id, city_id, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#;

        sqlx::query(query)
            .bind(school.id)
            .bind(&school.name)
            .bind(&school.address)
            .bind(&school.logo_path)
            .bind(school.subscription_id)
            .bind(&school.province_id)
            .bind(&school.city_id)
            .bind(school.created_at)
            .bind(school.updated_at)
            .bind(school.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn update(&self, school: &School) -> Result<(), Error> {
        let query = r#"
        UPDATE schools
            SET name = $1, address = $2, logo_path = $3, subscription_id = $4, province_id = $5, city_id = $6, updated_at = $7
            WHERE id = $8
        "#;

        sqlx::query(query)
            .bind(&school.name)
            .bind(&school.address)
            .bind(&school.logo_path)
            .bind(school.subscription_id)
            .bind(&school.province_id)
            .bind(&school.city_id)
            .bind(school.updated_at)
            .bind(school.id)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query = r#"
            DELETE FROM schools WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.database)
            .await?;

        Ok(())
    }
}
