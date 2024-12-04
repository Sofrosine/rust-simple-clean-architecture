use crate::helpers::custom_error::ErrorResponse;
use crate::internal::app::repositories::city_repository::{CityRepository, CityRepositoryImpl};
use crate::internal::app::repositories::province_repository::{ProvinceRepository, ProvinceRepositoryImpl};
use crate::internal::entities::city::{City, CityDataResponse};
use actix_web::http::StatusCode;
use chrono::Utc;
use std::fmt::Debug;

pub trait CityUseCase {
    fn new(repository: CityRepositoryImpl, province_repository: ProvinceRepositoryImpl) -> Self;
    async fn list(&self) -> Result<Vec<City>, ErrorResponse>;
    async fn create(&self) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct CityUseCaseImpl {
    repository: CityRepositoryImpl,
    province_repository: ProvinceRepositoryImpl,
}

impl CityUseCase for CityUseCaseImpl {
    fn new(repository: CityRepositoryImpl, province_repository: ProvinceRepositoryImpl) -> Self {
        Self { repository, province_repository }
    }

    async fn list(&self) -> Result<Vec<City>, ErrorResponse> {
        match self.repository.list().await {
            Ok(cities) => Ok(cities),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn create(&self) -> Result<(), ErrorResponse> {
        let provinces = match self.province_repository.list().await {
            Ok(provinces) => provinces,
            Err(error) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(error.to_string()),
                    Some("Failed to fetch provinces".to_string()),
                ));
            }
        };

        for province in provinces {
            let url = format!("https://wilayah.id/api/regencies/{}.json", province.id);

            // Fetch cities data for the current province
            let response = match reqwest::get(&url).await {
                Ok(res) => res.json::<CityDataResponse>().await.map_err(|err| {
                    ErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Some(err.to_string()),
                        Some("Failed to parse JSON response".to_string()),
                    )
                })?,
                Err(err) => {
                    eprintln!("Failed to fetch cities for province {}: {}", province.id, err);
                    continue; // Skip this province and proceed to the next one
                }
            };

            // Loop through the cities and insert or update them
            for city_data in response.data {
                let city = City {
                    code: city_data.code.clone(),
                    name: city_data.name.clone(),
                    province_id: province.id.clone(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                };

                // Check if the city exists by code
                match self.repository.get_by_id(city_data.code.clone()).await {
                    Ok(_) => {
                        // City exists, skip insertion
                        eprintln!("City with code {} already exists, skipping.", city_data.code.clone());
                    }
                    Err(sqlx::Error::RowNotFound) => {
                        // City does not exist, insert it
                        if let Err(error) = self.repository.create(&city).await {
                            eprintln!("Error inserting city {:?}: {}", city, error);
                        }
                    }
                    Err(error) => {
                        // Handle unexpected errors from the database
                        eprintln!(
                            "Error checking existence of city with code {}: {}",
                            city.code, error
                        );
                    }
                }
            }
        }

        Ok(())
    }

}