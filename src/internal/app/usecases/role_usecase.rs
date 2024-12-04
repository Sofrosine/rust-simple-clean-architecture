use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::Utc;
use crate::helpers::custom_error::ErrorResponse;
use crate::internal::app::repositories::role_repository::{RoleRepository, RoleRepositoryImpl};
use crate::internal::entities::role::Role;
use crate::pkg::dto::role_dto::{CreateRoleDto, UpdateRoleDto};

pub trait RoleUseCase {
    fn new(repository: RoleRepositoryImpl) -> Self;
    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<Role>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateRoleDto>) -> Result<(), ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateRoleDto>) -> Result<(), ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct RoleUseCaseImpl {
    repository: RoleRepositoryImpl,
}

impl RoleUseCase for RoleUseCaseImpl {
    fn new(repository: RoleRepositoryImpl) -> Self {
        Self { repository }
    }

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<Role>, i64), ErrorResponse> {
        if page == 0 || page_size == 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid pagination parameters".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let offset = (page - 1) * page_size;

        match self.repository.list(offset, page_size).await {
            Ok((roles, total_data)) => Ok((roles, total_data)),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn create(&self, form: Json<CreateRoleDto>) -> Result<(), ErrorResponse> {
        let CreateRoleDto {
            name,
        } = form.into_inner();

        if name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid role name".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let role = Role {
            id: uuid::Uuid::new_v4(),
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&role).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        }
    }

    async fn update(&self, id: String, form: Json<UpdateRoleDto>) -> Result<(), ErrorResponse> {
        let UpdateRoleDto {
            name
        } = form.into_inner();

        let mut role = match self.repository.get_by_id(id.parse().unwrap()).await {
            Ok(role) => role,
            Err(error) => return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        };

        role.name = name.unwrap_or(role.name).trim().to_string();
        role.updated_at = Utc::now();


        match self.repository.update(&role).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        }
    }

    async fn delete(&self, id: String) -> Result<(), ErrorResponse> {
        match self.repository.delete(id.parse().unwrap()).await {
            Ok(()) => Ok(()),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            ))
        }
    }
}