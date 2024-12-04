use crate::helpers::custom_error::ErrorResponse;
use crate::internal::app::repositories::province_repository::{ProvinceRepository, ProvinceRepositoryImpl};
use crate::internal::entities::province::{Province, ProvinceDataResponse, ProvinceFromTable};
use actix_web::http::StatusCode;
use chrono::Utc;
use std::fmt::Debug;

pub trait ProvinceUseCase {
    fn new(repository: ProvinceRepositoryImpl) -> Self;
    async fn list(&self) -> Result<Vec<ProvinceFromTable>, ErrorResponse>;
    async fn create(&self) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct ProvinceUseCaseImpl {
    repository: ProvinceRepositoryImpl,
}

impl ProvinceUseCase for ProvinceUseCaseImpl {
    fn new(repository: ProvinceRepositoryImpl) -> Self {
        Self { repository }
    }

    async fn list(&self) -> Result<Vec<ProvinceFromTable>, ErrorResponse> {
        match self.repository.list().await {
            Ok(provinces) => Ok(provinces),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn create(&self) -> Result<(), ErrorResponse> {
        let url = "https://wilayah.id/api/provinces.json";

        // Fetch provinces data from the API
        let response = match reqwest::get(url).await {
            Ok(res) => res.json::<ProvinceDataResponse>().await.map_err(|err| {
                ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(err.to_string()),
                    Some("Failed to parse JSON response".to_string()),
                )
            })?,
            Err(err) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(err.to_string()),
                    Some("Failed to fetch provinces".to_string()),
                ));
            }
        };

        // Loop through the provinces and insert or update them
        for province_data in response.data {
            let province = Province {
                code: province_data.code.clone(),
                name: province_data.name.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };

            // Check if the province exists by code
            // Check if the province exists by code
            match self.repository.get_by_id(province_data.code.clone()).await {
                Ok(_) => {
                    // Province exists, skip insertion
                    eprintln!("Province with code {} already exists, skipping.", province_data.code.clone());
                }
                Err(sqlx::Error::RowNotFound) => {
                    // Province does not exist, insert it
                    if let Err(error) = self.repository.create(&province).await {
                        eprintln!("Error inserting province {:?}: {}", province, error);
                    }
                }
                Err(error) => {
                    // Handle unexpected errors from the database
                    eprintln!(
                        "Error checking existence of province with code {}: {}",
                        province.code, error
                    );
                }
            }
        }

        Ok(())
    }
}