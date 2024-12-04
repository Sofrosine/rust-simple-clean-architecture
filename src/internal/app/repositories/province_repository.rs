use crate::internal::entities::province::{Province, ProvinceFromTable};
use sqlx::{query_as, Error, PgPool};

pub trait ProvinceRepository {
    fn new(database: PgPool) -> Self;

    async fn list(&self) -> Result<Vec<ProvinceFromTable>, Error>;
    async fn get_by_id(&self, id: String) -> Result<Province, Error>;
    async fn create(&self, province: &Province) -> Result<(), Error>;
    // async fn update(&self, province: &Province) -> Result<(), Error>;

}

#[derive(Debug, Clone)]
pub struct ProvinceRepositoryImpl {
    database: PgPool,
}

impl ProvinceRepository for ProvinceRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self) -> Result<Vec<ProvinceFromTable>, Error> {
        let query = r#"
            SELECT * FROM provinces ORDER BY name
        "#;


        let rows = query_as(query)
            .fetch_all(&self.database)
            .await?;


        Ok(rows)
    }

    async fn get_by_id(&self, id: String) -> Result<Province, Error> {
        let query = r#"
        SELECT * FROM provinces WHERE id = $1
    "#;

        let province = query_as::<_, Province>(query)
            .bind(id)
            .fetch_one(&self.database)
            .await?;

        Ok(province)
    }

    async fn create(&self, province: &Province) -> Result<(), Error> {
        let query = r#"
            INSERT INTO provinces (id, name, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        sqlx::query(query)
            .bind(&province.code)
            .bind(&province.name)
            .bind(province.created_at)
            .bind(province.updated_at)
            .bind(province.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    // async fn update(&self, province: &Province) -> Result<(), Error> {
    //     let query = r#"
    //     UPDATE provinces
    //         SET name = $1, updated_at = $2
    //         WHERE id = $3
    //     "#;
    //
    //     sqlx::query(query)
    //         .bind(&province.name)
    //         .bind(province.updated_at)
    //         .bind(&province.code)
    //         .execute(&self.database)
    //         .await?;
    //
    //     Ok(())
    // }
}