use actix_web::web;
use crate::internal::handlers::auth_handler::{register, AuthHandlerImpl};

pub fn auth_router(conf: &mut web::ServiceConfig, handler: AuthHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/auth")
                .route("/register", web::post().to(register))
        );
}