use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Query;
use serde_json::json;
use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::{PaginatedResponse, PaginationParams};
use crate::internal::app::usecases::role_usecase::{RoleUseCase, RoleUseCaseImpl};
use crate::pkg::dto::role_dto::{CreateRoleDto, UpdateRoleDto};

#[derive(Clone)]
pub struct RoleHandlerImpl {
    service: RoleUseCaseImpl,
}

impl RoleHandlerImpl {
    pub fn new(service: RoleUseCaseImpl) -> Self {
        Self { service }
    }
}

pub async fn role_handler_list(handler: web::Data<RoleHandlerImpl>, params: Query<PaginationParams>) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match handler.service.list(page, page_size).await {
        Ok((roles, total_data)) => {
            let total_pages = (total_data as f32 / page_size as f32).ceil() as u32;
            let response = PaginatedResponse {
                data: roles,
                page_size,
                page,
                total_pages,
                total_data: total_data as u32,
            };
            HttpResponse::Ok().json(json!({
            "data": response,
            "message": "Successfully fetched roles",
            "code": 200
        }))
        }
        Err(err) => err.error_response(),
    }
}

pub async fn role_handler_create(
    handler: web::Data<RoleHandlerImpl>,
    input: web::Json<CreateRoleDto>,
) -> impl Responder {
    match handler.service.create(input).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "Role created successfully",
            "code": 201
        })),
        Err(err) => err.error_response()
    }
}

pub async fn role_handler_update(
    handler: web::Data<RoleHandlerImpl>,
    path: web::Path<String>,
    input: web::Json<UpdateRoleDto>,
) -> impl Responder {
    let path_id = path.into_inner();
    match handler.service.update(path_id, input).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Role updated successfully",
            "code": 200
        })),
        Err(err) => err.error_response()
    }
}

pub async fn role_handler_delete(
    handler: web::Data<RoleHandlerImpl>,
    path: web::Path<String>,
) -> impl Responder {
    let path_id = path.into_inner();
    match handler.service.delete(path_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Role deleted successfully",
            "code": 200
        })),
        Err(err) => err.error_response()
    }
}