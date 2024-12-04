use crate::internal::handlers::school_handler::{school_handler_create, school_handler_delete, school_handler_list, school_handler_update, SchoolHandlerImpl};
use actix_web::web;

pub fn school_router(conf: &mut web::ServiceConfig, handler: SchoolHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/schools")
                .route("", web::get().to(school_handler_list))
                .route("", web::post().to(school_handler_create))
                .route("/{id}", web::put().to(school_handler_update))
                .route("/{id}", web::delete().to(school_handler_delete))
        );
}