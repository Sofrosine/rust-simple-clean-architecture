use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;
use crate::internal::entities::role::Role;

pub trait RoleRepository {
        fn new(database: PgPool) -> Self;

        async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<Role>, i64), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Role, Error>;
    async fn get_by_name(&self, name: String) -> Result<Role, Error>;
    async fn create(&self, role: &Role) -> Result<(), Error>;
    async fn update(&self, role: &Role) -> Result<(), Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct RoleRepositoryImpl {
    database: PgPool,
}

impl RoleRepository for RoleRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<Role>, i64), Error> {
        let query = r#"
            SELECT * FROM roles ORDER BY name ASC LIMIT $1 OFFSET $2
        "#;

        let count_query = r#"
            SELECT COUNT(*) AS total FROM roles
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

    async fn get_by_id(&self, id: Uuid) -> Result<Role, Error> {
        let query = r#"
            SELECT * FROM roles WHERE id = $1 AND deleted_at IS NULL
        "#;

        let role = query_as(query).bind(id).fetch_one(&self.database).await?;

        Ok(role)
    }

    async fn get_by_name(&self, name: String) -> Result<Role, Error> {
        let query = r#"
            SELECT * FROM roles WHERE name = $1 AND deleted_at IS NULL
        "#;

        let role = query_as(query).bind(name).fetch_one(&self.database).await?;

        Ok(role)
    }

    async fn create(&self, role: &Role) -> Result<(), Error> {
        let query = r#"
            INSERT INTO roles (id, name, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        sqlx::query(query)
            .bind(role.id)
            .bind(&role.name)
            .bind(role.created_at)
            .bind(role.updated_at)
            .bind(role.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn update(&self, role: &Role) -> Result<(), Error> {
        let query = r#"
        UPDATE roles
            SET name = $1, updated_at = $2
            WHERE id = $3
        "#;

        sqlx::query(query)
            .bind(&role.name)
            .bind(role.updated_at)
            .bind(role.id)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query = r#"
            DELETE FROM roles WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.database)
            .await?;


        Ok(())
    }
}