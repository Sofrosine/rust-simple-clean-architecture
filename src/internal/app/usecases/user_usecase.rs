use crate::internal::app::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::internal::entities::user::{User, UserStatus};
use crate::helpers::custom_error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use uuid::Uuid;
use crate::internal::app::repositories::role_repository::{RoleRepository, RoleRepositoryImpl};
use crate::internal::app::repositories::school_repository::{SchoolRepository, SchoolRepositoryImpl};
use crate::pkg::dto::user_dto::{CreateUserDto, UpdateUserDto};

pub trait UserUseCase {
    fn new(
        repository: UserRepositoryImpl,
        role_repository: RoleRepositoryImpl,
        school_repository: SchoolRepositoryImpl,
    ) -> Self;

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<User>, i64), ErrorResponse>;
    async fn create(&self, form: Json<CreateUserDto>) -> Result<User, ErrorResponse>;
    async fn update(&self, id: String, form: Json<UpdateUserDto>) -> Result<User, ErrorResponse>;
    async fn delete(&self, id: String) -> Result<(), ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct UserUseCaseImpl {
    repository: UserRepositoryImpl,
    role_repository: RoleRepositoryImpl,
    school_repository: SchoolRepositoryImpl,
}

impl UserUseCase for UserUseCaseImpl {
    fn new(
        repository: UserRepositoryImpl,
        role_repository: RoleRepositoryImpl,
        school_repository: SchoolRepositoryImpl,
    ) -> Self {
        Self {
            repository,
            role_repository,
            school_repository,
        }
    }

    async fn list(&self, page: u32, page_size: u32) -> Result<(Vec<User>, i64), ErrorResponse> {
        if page == 0 || page_size == 0 {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Invalid pagination parameters".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let offset = (page - 1) * page_size;

        match self.repository.list(offset, page_size).await {
            Ok((users, total_data)) => Ok((users, total_data)),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn create(&self, form: Json<CreateUserDto>) -> Result<User, ErrorResponse> {
        let CreateUserDto {
            name,
            email,
            phone_number,
            password,
            title,
            role_id,
            school_id,
        } = form.into_inner();

        // Validate required fields.
        if name.trim().is_empty() || email.trim().is_empty() || phone_number.trim().is_empty() || password.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Missing required fields".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        // Validate role existence.
        if let Some(role_id) = role_id {
            if self.role_repository.get_by_id(role_id).await.is_err() {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(format!("Role with ID {} does not exist", role_id)),
                    Some("FAILED".to_string()),
                ));
            }
        }

        // Validate school existence.
        if let Some(school_id) = school_id {
            if self.school_repository.get_by_id(school_id).await.is_err() {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(format!("School with ID {} does not exist", school_id)),
                    Some("FAILED".to_string()),
                ));
            }
        }

        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some("Failed to hash password".to_string()),
                    Some("FAILED".to_string()),
                ));
            }
        };

        // Create the user entity.
        let user = User {
            id: Uuid::new_v4(),
            name,
            email,
            phone_number,
            password: hashed_password,
            title: title.unwrap_or_default(),
            status: UserStatus::Pending,
            role_id: role_id.unwrap_or_default(),
            school_id: school_id.unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.create(&user).await {
            Ok(user) => Ok(user),
            Err(error) => Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )),
        }
    }

    async fn update(&self, id: String, form: Json<UpdateUserDto>) -> Result<User, ErrorResponse> {
        let UpdateUserDto {
            name,
            email,
            phone_number,
            password,
            title,
            status,
            role_id,
            school_id,
        } = form.into_inner();

        // Validate role existence.
        if let Some(role_id) = role_id {
            if self.role_repository.get_by_id(role_id).await.is_err() {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(format!("Role with ID {} does not exist", role_id)),
                    Some("FAILED".to_string()),
                ));
            }
        }

        // Validate school existence.
        if let Some(school_id) = school_id {
            if self.school_repository.get_by_id(school_id).await.is_err() {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(format!("School with ID {} does not exist", school_id)),
                    Some("FAILED".to_string()),
                ));
            }
        }

        // Fetch the existing user and prepare updated user entity.
        let user = self.repository.get_by_id(id.parse().unwrap()).await.map_err(|error| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(error.to_string()),
                Some("FAILED".to_string()),
            )
        })?;

        let hashed_password = if let Some(pwd) = password {
            // Hash the new password if provided
            match hash(pwd, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => {
                    return Err(ErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Some("Failed to hash password".to_string()),
                        Some("FAILED".to_string()),
                    ));
                }
            }
        } else {
            user.password // Keep the old password if no new one is provided
        };

        let updated_user = User {
            id: user.id,
            name: name.unwrap_or(user.name),
            email: email.unwrap_or(user.email),
            phone_number: phone_number.unwrap_or(user.phone_number),
            password: hashed_password,
            title: title.unwrap_or(user.title),
            status: status.unwrap_or(user.status),
            role_id: role_id.unwrap_or(user.role_id),
            school_id: school_id.unwrap_or(user.school_id),
            created_at: user.created_at,
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.repository.update(&updated_user).await {
            Ok(user) => Ok(user),
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
