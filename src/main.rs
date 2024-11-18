use crate::cmd::routes::subscription_router::subscription_router;
use crate::database::postgresql::get_pool;
use crate::internal::app::repositories::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};
use crate::internal::app::usecases::subscription_usecase::{SubscriptionUseCase, SubscriptionUseCaseImpl};
use crate::internal::handlers::subscription_handler::SubscriptionHandlerImpl;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;

mod database;
mod internal;
mod cmd;
mod pkg;
mod helpers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let pool = match get_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            println!("🔥 Failed to initialize the database pool: {:?}", err);
            std::process::exit(1);
        }
    };

    // Wrap the pool in an Arc to enable shared ownership
    let shared_pool = pool;


    let subscription_repository = SubscriptionRepositoryImpl::new(shared_pool.clone());
    let subscription_usecase = SubscriptionUseCaseImpl::new(subscription_repository);
    let subscription_handler = SubscriptionHandlerImpl::new(subscription_usecase);


    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .configure(|cfg| subscription_router(cfg, subscription_handler.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}