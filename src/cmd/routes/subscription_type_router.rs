use crate::internal::handlers::subscription_type_handler::{subscription_type_handler_create, subscription_type_handler_delete, subscription_type_handler_list, subscription_type_handler_update, SubscriptionTypeHandlerImpl};
use actix_web::web;

pub fn subscription_type_router(conf: &mut web::ServiceConfig, handler: SubscriptionTypeHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/subscription_types")
                .route("", web::get().to(subscription_type_handler_list))
                .route("", web::post().to(subscription_type_handler_create))
                .route("/{id}", web::put().to(subscription_type_handler_update))
                .route("/{id}", web::delete().to(subscription_type_handler_delete))
        );
}