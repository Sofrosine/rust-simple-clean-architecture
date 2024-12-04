use actix_web::web;
use crate::internal::handlers::province_handler::{province_handler_create, province_handler_list, ProvinceHandlerImpl};

pub fn province_router(conf: &mut web::ServiceConfig, handler: ProvinceHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/provinces")
                .route("", web::get().to(province_handler_list))
                .route("", web::post().to(province_handler_create))
        );
}