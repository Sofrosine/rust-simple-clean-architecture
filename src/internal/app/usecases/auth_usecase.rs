use actix_web::http::StatusCode;
use actix_web::web::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use uuid::Uuid;
use crate::helpers::custom_error::ErrorResponse;
use crate::internal::app::repositories::db_transaction_repository::{DbTransactionRepository, DbTransactionRepositoryImpl};
use crate::internal::app::repositories::role_repository::{RoleRepository, RoleRepositoryImpl};
use crate::internal::app::repositories::school_repository::{SchoolRepository, SchoolRepositoryImpl};
use crate::internal::app::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::internal::entities::school::School;
use crate::internal::entities::user::{User, UserStatus};
use crate::pkg::dto::auth_dto::RegisterDto;

pub trait AuthUseCase {
    fn new(
        user_repository: UserRepositoryImpl,
        role_repository: RoleRepositoryImpl,
        school_repository: SchoolRepositoryImpl,
        db_transaction_repository: DbTransactionRepositoryImpl,
    ) -> Self;

    async fn register(&self, form: Json<RegisterDto>) -> Result<User, ErrorResponse>;
}

#[derive(Debug, Clone)]
pub struct AuthUseCaseImpl {
    user_repository: UserRepositoryImpl,
    role_repository: RoleRepositoryImpl,
    school_repository: SchoolRepositoryImpl,
    db_transaction_repository: DbTransactionRepositoryImpl,
}

impl AuthUseCase for AuthUseCaseImpl {
    fn new(user_repository: UserRepositoryImpl, role_repository: RoleRepositoryImpl, school_repository: SchoolRepositoryImpl,
           db_transaction_repository: DbTransactionRepositoryImpl,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
            school_repository,
            db_transaction_repository,
        }
    }

    async fn register(&self, form: Json<RegisterDto>) -> Result<User, ErrorResponse> {
        let RegisterDto {
            name,
            email,
            phone_number,
            password,
            school_name
        } = form.into_inner();

        if name.trim().is_empty() || email.trim().is_empty() || phone_number.trim().is_empty() || password.trim().is_empty() || school_name.trim().is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Missing required fields".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        if self.user_repository.get_by_email(email.clone()).await.is_ok() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Email is already used".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        if self.user_repository.get_by_phone(phone_number.clone()).await.is_ok() {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some("Phone is already used".to_string()),
                Some("FAILED".to_string()),
            ));
        }

        let tx = match self.db_transaction_repository.begin_transaction().await {
            Ok(transaction) => transaction,
            Err(err) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(err.to_string()),
                    Some("FAILED".to_string()),
                ))
            }
        };


        let user_role = match self.role_repository.get_by_name("user".to_string()).await {
            Ok(role) => role,
            Err(err) => {
                tx.rollback().await.unwrap();
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(err.to_string()),
                    Some("FAILED".to_string()),
                ));
            }
        };

        let school = School {
            id: Uuid::new_v4(),
            name: school_name,
            address: "".to_string(),
            logo_path: "".to_string(),
            subscription_id: None,
            province_id: None,
            city_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let school = match self.school_repository.create(&school).await {
            Ok((school)) => school,
            Err(err) => {
                tx.rollback().await.unwrap();
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some(err.to_string()),
                    Some("FAILED".to_string()),
                ));
            }
        };


        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => {
                tx.rollback().await.unwrap();
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
            title: "".to_string(),
            status: UserStatus::Pending,
            role_id: user_role.id,
            school_id: school.id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        match self.user_repository.create(&user).await {
            Ok(user) => {
                tx.commit().await.unwrap();
                Ok(user)
            },
            Err(error) => {
                tx.rollback().await.unwrap();
                Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(error.to_string()),
                    Some("FAILED".to_string()),
                ))
            }
        }
    }
}