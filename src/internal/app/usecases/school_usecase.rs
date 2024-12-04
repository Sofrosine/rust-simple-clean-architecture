use crate::internal::app::repositories::school_repository::{SchoolRepository, SchoolRepositoryImpl};
use crate::internal::entities::school::School;
use crate::pkg::dto::school_dto::{CreateSchoolDto, UpdateSchoolDto};
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use std::fmt::Debug;

pub trait SchoolUseCase {
    fn new(repository: SchoolRepositoryImpl) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<School>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateSchoolDto>) -> Result<(), ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateSchoolDto>) -> Result<(), ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct SchoolUseCaseImpl {
    repository: SchoolRepositoryImpl,
}

impl SchoolUseCase for SchoolUseCaseImpl {
    fn new(repository: SchoolRepositoryImpl) -> Self {
        Self { repository }
    }

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<School>, i64), ErrorResponse> {
        if page == 0 || page_size == 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid pagination parameters".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let offset = (page - 1) * page_size;

        match self.repository.list(offset, page_size).await {
            Ok((schools, total_data)) => Ok((schools, total_data)),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn create(&self, form: Json<CreateSchoolDto>) -> Result<(), ErrorResponse> {
        let CreateSchoolDto {
            name,
            address,
            logo_path,
            subscription_id,
            province_id,
            city_id,
        } = form.into_inner();

        // Validate input
        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid school name".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        // Create school
        let school = School {
            id: uuid::Uuid::new_v4(),
            name,
            address: address.unwrap_or_default(),
            logo_path: logo_path.unwrap_or_default(),
            subscription_id,
            province_id,
            city_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&school).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn update(&self, id: String, form: Json<UpdateSchoolDto>) -> Result<(), ErrorResponse> {
        let UpdateSchoolDto {
            name,
            address,
            logo_path,
            subscription_id,
            province_id,
            city_id,
        } = form.into_inner();

        let school = match self.repository.get_by_id(id.parse().unwrap()).await {
            Ok(school) => school,
            Err(error) => return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        };

        let updated_school = School {
            id: school.id,
            name: name.unwrap_or(school.name),
            address: address.unwrap_or(school.address),
            logo_path: logo_path.unwrap_or(school.logo_path),
            subscription_id,
            province_id,
            city_id,
            created_at: school.created_at,
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.update(&updated_school).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn delete(&self, id: String) -> Result<(), ErrorResponse> {
        match self.repository.delete(id.parse().unwrap()).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }
}
