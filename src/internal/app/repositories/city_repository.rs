use sqlx::{query_as, Error, PgPool};
use crate::internal::entities::city::City;

pub trait CityRepository {
    fn new(database: PgPool) -> Self;

    async fn list(&self) -> Result<Vec<City>, Error>;
    async fn get_by_id(&self, id: String) -> Result<City, Error>;
    async fn create(&self, city: &City) -> Result<(), Error>;
    // async fn update(&self, city: &City) -> Result<(), Error>;

}

#[derive(Debug, Clone)]
pub struct CityRepositoryImpl {
    database: PgPool,
}

impl CityRepository for CityRepositoryImpl {
    fn new(database: PgPool) -> Self {
        Self { database }
    }

    async fn list(&self) -> Result<Vec<City>, Error> {
        let query = r#"
            SELECT * FROM cities ORDER BY name
        "#;


        let rows = query_as(query)
            .fetch_all(&self.database)
            .await?;


        Ok(rows)
    }

    async fn get_by_id(&self, id: String) -> Result<City, Error> {
        let query = r#"
        SELECT * FROM cities WHERE id = $1
    "#;

        let city = query_as::<_, City>(query)
            .bind(id)
            .fetch_one(&self.database)
            .await?;

        Ok(city)
    }

    async fn create(&self, city: &City) -> Result<(), Error> {
        let query = r#"
            INSERT INTO cities (id, name, province_id, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        sqlx::query(query)
            .bind(&city.code)
            .bind(&city.name)
            .bind(&city.province_id)
            .bind(city.created_at)
            .bind(city.updated_at)
            .bind(city.deleted_at)
            .execute(&self.database)
            .await?;

        Ok(())
    }

    // async fn update(&self, city: &City) -> Result<(), Error> {
    //     let query = r#"
    //     UPDATE cities
    //         SET name = $1, updated_at = $2
    //         WHERE id = $3
    //     "#;
    //
    //     sqlx::query(query)
    //         .bind(&city.name)
    //         .bind(city.updated_at)
    //         .bind(&city.code)
    //         .execute(&self.database)
    //         .await?;
    //
    //     Ok(())
    // }
}