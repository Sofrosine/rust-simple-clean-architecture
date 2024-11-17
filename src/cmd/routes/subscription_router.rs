use crate::internal::handlers::subscription_handler::{subscription_handler_create, subscription_handler_list, SubscriptionHandlerImpl};
use actix_web::web;

pub fn subscription_router(conf: &mut web::ServiceConfig, handler: SubscriptionHandlerImpl) {
    conf.app_data(web::Data::new(handler))
        .service(
            web::scope("/subscriptions")
                .route("", web::get().to(subscription_handler_list))
                .route("", web::post().to(subscription_handler_create))
        );
}