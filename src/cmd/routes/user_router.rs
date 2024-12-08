use actix_web::middleware::from_fn;
use actix_web::web;
use crate::cmd::middlewares::auth::{super_admin_middleware};
use crate::internal::handlers::user_handler::{user_handler_create, user_handler_delete, user_handler_list, user_handler_update, UserHandlerImpl};

pub fn user_router(conf: &mut web::ServiceConfig, handler: UserHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/users")
                .wrap(from_fn(super_admin_middleware))
                // .wrap(from_fn(|req, next| role_middleware(req, next, vec!["admin".to_string()])))
                .route("", web::get().to(user_handler_list))
                .route("", web::post().to(user_handler_create))
                .route("/{id}", web::put().to(user_handler_update))
                .route("/{id}", web::delete().to(user_handler_delete))
        );
}