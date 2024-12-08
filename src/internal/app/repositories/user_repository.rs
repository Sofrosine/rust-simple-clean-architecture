use sqlx::{query_as, Error, PgPool};
use uuid::Uuid;
use crate::internal::entities::user::User;

pub trait UserRepository {
    fn new(database: PgPool) -> Self;
    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<User>, i64), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<User, Error>;
    async fn get_by_email(&self, email: String) -> Result<User, Error>;
    async fn get_by_phone(&self, phone_number: String) -> Result<User, Error>;
    async fn create(&self, user: &User) -> Result<User, Error>;
    async fn update(&self, user: &User) -> Result<User, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    database: PgPool,
}

impl UserRepository for UserRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self, offset: u32, page_size: u32) -> Result<(Vec<User>, i64), Error> {
        let query = r#"
            SELECT * FROM users WHERE deleted_at IS NULL ORDER BY created_at ASC LIMIT $1 OFFSET $2
        "#;

        let count_query = r#"
            SELECT COUNT(*) AS total FROM users WHERE deleted_at IS NULL
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

    async fn get_by_id(&self, id: Uuid) -> Result<User, Error> {
        let query = r#"
            SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL
        "#;

        let user = query_as(query).bind(id).fetch_one(&self.database).await?;

        Ok(user)
    }

    async fn get_by_email(&self, email: String) -> Result<User, Error> {
        let query = r#"
            SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL
        "#;

        let user = query_as(query).bind(email).fetch_one(&self.database).await?;

        Ok(user)
    }

    async fn get_by_phone(&self, phone_number: String) -> Result<User, Error> {
        let query = r#"
            SELECT * FROM users WHERE phone_number = $1 AND deleted_at IS NULL
        "#;

        let user = query_as(query).bind(phone_number).fetch_one(&self.database).await?;

        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<User, Error> {
        let query = r#"
            INSERT INTO users (id, name, email, phone_number, password, title, role_id, school_id, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
        "#;

        let created_user = sqlx::query_as::<_, User>(query)
            .bind(user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.phone_number)
            .bind(&user.password)
            .bind(&user.title)
            .bind(user.role_id)
            .bind(user.school_id)
            .bind(user.created_at)
            .bind(user.updated_at)
            .bind(user.deleted_at)
            .fetch_one(&self.database)
            .await?;

        Ok(created_user)
    }

    async fn update(&self, user: &User) -> Result<User, Error> {
        let query = r#"
            UPDATE users
            SET name = $1, email = $2, phone_number = $3, password = $4, title = $5, role_id = $6, school_id = $7, updated_at = $8
            WHERE id = $9 AND deleted_at IS NULL
            RETURNING id, name, email, phone_number, password, title, role_id, school_id, created_at, updated_at, deleted_at
        "#;

        let updated_user = sqlx::query_as::<_, User>(query)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.phone_number)
            .bind(&user.password)
            .bind(&user.title)
            .bind(user.role_id)
            .bind(user.school_id)
            .bind(user.updated_at)
            .bind(user.id)
            .fetch_one(&self.database)
            .await?;

        Ok(updated_user)
    }


    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query = r#"
            DELETE FROM users WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.database)
            .await?;

        Ok(())
    }
}
