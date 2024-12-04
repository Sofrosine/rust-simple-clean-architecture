use actix_web::web;
use crate::internal::handlers::city_handler::{city_handler_create, city_handler_list, CityHandlerImpl};
pub fn city_router(conf: &mut web::ServiceConfig, handler: CityHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/cities")
                .route("", web::get().to(city_handler_list))
                .route("", web::post().to(city_handler_create))
        );
}