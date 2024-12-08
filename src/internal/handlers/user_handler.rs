use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Query;
use serde_json::json;
use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::{PaginatedResponse, PaginationParams};
use crate::internal::app::usecases::user_usecase::{UserUseCase, UserUseCaseImpl};
use crate::pkg::dto::user_dto::{CreateUserDto, UpdateUserDto};

#[derive(Clone)]
pub struct UserHandlerImpl {
    service: UserUseCaseImpl,
}

impl UserHandlerImpl {
    pub fn new(service: UserUseCaseImpl) -> Self {
        Self { service }
    }
}

// Handler for listing users
pub async fn user_handler_list(
    handler: web::Data<UserHandlerImpl>,
    params: Query<PaginationParams>,
) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match handler.service.list(page, page_size).await {
        Ok((users, total_data)) => {
            let response = PaginatedResponse {
                data: users,
                page_size,
                page,
                total_pages: (total_data as f32 / page_size as f32).ceil() as u32,
                total_data: total_data as u32,
            };

            HttpResponse::Ok().json(json!({
                "data": response,
                "message": "Successfully fetched users",
                "code": 200
            }))
        }
        Err(err) => err.error_response(),
    }
}

// Handler for creating a user
pub async fn user_handler_create(
    handler: web::Data<UserHandlerImpl>,
    input: web::Json<CreateUserDto>,
) -> impl Responder {
    match handler.service.create(input).await {
        Ok(user) => HttpResponse::Created().json(json!({
            "data": user,
            "message": "User created successfully",
            "code": 201
        })),
        Err(err) => err.error_response(),
    }
}

// Handler for updating a user
pub async fn user_handler_update(
    handler: web::Data<UserHandlerImpl>,
    path: web::Path<String>,
    input: web::Json<UpdateUserDto>,
) -> impl Responder {
    let user_id = path.into_inner();

    match handler.service.update(user_id, input).await {
        Ok(user) => HttpResponse::Ok().json(json!({
            "data": user,
            "message": "User updated successfully",
            "code": 200
        })),
        Err(err) => err.error_response(),
    }
}

// Handler for deleting a user
pub async fn user_handler_delete(
    handler: web::Data<UserHandlerImpl>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = path.into_inner();

    match handler.service.delete(user_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "User deleted successfully",
            "code": 200
        })),
        Err(err) => err.error_response(),
    }
}
