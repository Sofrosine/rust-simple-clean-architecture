use actix_multipart::form::MultipartForm;
use crate::internal::app::usecases::school_usecase::{SchoolUseCase, SchoolUseCaseImpl};
use crate::pkg::dto::school_dto::{CreateSchoolDto, UpdateSchoolDto};
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Query;
use serde_json::json;
use crate::helpers::custom_error::ResponseError;
use crate::helpers::custom_response::{PaginatedResponse, PaginationParams};

#[derive(Clone)]
pub struct SchoolHandlerImpl {
    service: SchoolUseCaseImpl,
}

impl SchoolHandlerImpl {
    pub fn new(service: SchoolUseCaseImpl) -> Self {
        Self { service }
    }
}

// Handler for listing schools
pub async fn school_handler_list(
    handler: web::Data<SchoolHandlerImpl>,
    params: Query<PaginationParams>,
) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match handler.service.list(page, page_size).await {
        Ok((schools, total_data)) => {
            let response = PaginatedResponse {
                data: schools,
                page_size,
                page,
                total_pages: (total_data as f32 / page_size as f32).ceil() as u32,
                total_data: total_data as u32,
            };

            HttpResponse::Ok().json(json!({
                "data": response,
                "message": "Successfully fetched schools",
                "code": 200
            }))
        }
        Err(err) => err.error_response(),
    }
}

// Handler for creating a school
pub async fn school_handler_create(
    handler: web::Data<SchoolHandlerImpl>,
    input: MultipartForm<CreateSchoolDto>,
) -> impl Responder {
    match handler.service.create(input).await {
        Ok(school) => HttpResponse::Created().json(json!({
            "data": school,
            "message": "School created successfully",
            "code": 201
        })),
        Err(err) => err.error_response(),
    }
}

// Handler for updating a school
pub async fn school_handler_update(
    handler: web::Data<SchoolHandlerImpl>,
    path: web::Path<String>,
    input: web::Json<UpdateSchoolDto>,
) -> impl Responder {
    let school_id = path.into_inner();

    match handler.service.update(school_id, input).await {
        Ok(school) => HttpResponse::Ok().json(json!({
            "data": school,
            "message": "School updated successfully",
            "code": 200
        })),
        Err(err) => err.error_response(),
    }
}

// Handler for deleting a school
pub async fn school_handler_delete(
    handler: web::Data<SchoolHandlerImpl>,
    path: web::Path<String>,
) -> impl Responder {
    let school_id = path.into_inner();

    match handler.service.delete(school_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "School deleted successfully",
            "code": 200
        })),
        Err(err) => err.error_response(),
    }
}
