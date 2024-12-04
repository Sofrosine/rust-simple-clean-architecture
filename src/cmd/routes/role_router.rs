use actix_web::web;
use crate::internal::handlers::role_handler::{role_handler_create, role_handler_delete, role_handler_list, role_handler_update, RoleHandlerImpl};

pub fn role_router(conf: &mut web::ServiceConfig, handler: RoleHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/roles")
                .route("", web::get().to(role_handler_list))
                .route("", web::post().to(role_handler_create))
                .route("/{id}", web::put().to(role_handler_update))
                .route("/{id}", web::delete().to(role_handler_delete))
        );
}