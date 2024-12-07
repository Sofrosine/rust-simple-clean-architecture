use crate::internal::app::repositories::school_repository::{SchoolRepository, SchoolRepositoryImpl};
use crate::internal::entities::school::School;
use crate::pkg::dto::school_dto::{CreateSchoolDto, UpdateSchoolDto};
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use std::fmt::Debug;

use aws_sdk_s3::Client;
use actix_multipart::form::MultipartForm;
use uuid::Uuid;
use crate::pkg::s3::upload_file_to_s3;
// Method to upload a logo to S3

pub trait SchoolUseCase {
    fn new(repository: SchoolRepositoryImpl, s3_client: Client) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<School>, i64), ErrorResponse>;
    async fn create(&self, form: MultipartForm<CreateSchoolDto>) -> Result<(), ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateSchoolDto>) -> Result<(), ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct SchoolUseCaseImpl {
    repository: SchoolRepositoryImpl,
    s3_client: Client,
}

impl SchoolUseCase for SchoolUseCaseImpl {
    fn new(repository: SchoolRepositoryImpl, s3_client: Client) -> Self {
        Self { repository, s3_client }
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

    async fn create(&self, form: MultipartForm<CreateSchoolDto>) -> Result<(), ErrorResponse> {
        let CreateSchoolDto {
            name,
            address,
            subscription_id,
            province_id,
            city_id,
            logo,
        } = form.into_inner();

        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid school name".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let address_str = address.map_or_else(|| "".to_string(), |addr| addr.to_string());
        let province_id_option = province_id.map_or_else(|| None, |id| Some(id.to_string()));
        let city_id_option = city_id.map_or_else(|| None, |id| Some(id.to_string()));
        let subscription_id_option = subscription_id.map_or(None, |id| Some(id.to_string().parse().unwrap_or_default()));


        let file_path = format!("school-logo/{}.{}", Uuid::new_v4(), "png");

        // Upload file to S3 and get the path
        match upload_file_to_s3(self.s3_client.clone(), logo, file_path.clone()).await {
            Ok(path) => path,
            Err(e) => return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("Failed to upload logo: {}", e)),
                Some("FAILED".to_string()),
            )),
        };

        // Create school
        let school = School {
            id: Uuid::new_v4(),
            name: name.to_string(),
            address: address_str,
            logo_path: file_path.clone(),
            subscription_id: subscription_id_option,
            province_id: province_id_option,
            city_id: city_id_option,
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
