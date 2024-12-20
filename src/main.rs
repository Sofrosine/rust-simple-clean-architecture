use crate::cmd::routes::subscription_router::subscription_router;
use crate::database::postgresql::get_pool;
use crate::internal::app::repositories::subscription_repository::{SubscriptionRepository, SubscriptionRepositoryImpl};
use crate::internal::app::usecases::subscription_usecase::{SubscriptionUseCase, SubscriptionUseCaseImpl};
use crate::internal::handlers::subscription_handler::SubscriptionHandlerImpl;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use crate::cmd::routes::auth::auth_router;
use crate::cmd::routes::city_router::city_router;
use crate::cmd::routes::province_router::province_router;
use crate::cmd::routes::role_router::role_router;
use crate::cmd::routes::school_router::school_router;
use crate::cmd::routes::subscription_type_router::subscription_type_router;
use crate::cmd::routes::user_router::user_router;
use crate::internal::app::repositories::city_repository::{CityRepository, CityRepositoryImpl};
use crate::internal::app::repositories::db_transaction_repository::DbTransactionRepositoryImpl;
use crate::internal::app::repositories::province_repository::{ProvinceRepository, ProvinceRepositoryImpl};
use crate::internal::app::repositories::role_repository::{RoleRepository, RoleRepositoryImpl};
use crate::internal::app::repositories::school_repository::{SchoolRepository, SchoolRepositoryImpl};
use crate::internal::app::repositories::subscription_type_repository::{SubscriptionTypeRepository, SubscriptionTypeRepositoryImpl};
use crate::internal::app::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::internal::app::usecases::auth_usecase::{AuthUseCase, AuthUseCaseImpl};
use crate::internal::app::usecases::city_usecase::{CityUseCase, CityUseCaseImpl};
use crate::internal::app::usecases::province_usecase::{ProvinceUseCase, ProvinceUseCaseImpl};
use crate::internal::app::usecases::role_usecase::{RoleUseCase, RoleUseCaseImpl};
use crate::internal::app::usecases::school_usecase::{SchoolUseCase, SchoolUseCaseImpl};
use crate::internal::app::usecases::subscription_type_usecase::{SubscriptionTypeUseCase, SubscriptionTypeUseCaseImpl};
use crate::internal::app::usecases::user_usecase::{UserUseCase, UserUseCaseImpl};
use crate::internal::handlers::auth_handler::AuthHandlerImpl;
use crate::internal::handlers::city_handler::CityHandlerImpl;
use crate::internal::handlers::province_handler::ProvinceHandlerImpl;
use crate::internal::handlers::role_handler::RoleHandlerImpl;
use crate::internal::handlers::school_handler::SchoolHandlerImpl;
use crate::internal::handlers::subscription_type_handler::SubscriptionTypeHandlerImpl;
use crate::internal::handlers::user_handler::UserHandlerImpl;
use crate::pkg::s3::create_s3_client;

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

    let s3_client = create_s3_client().await.unwrap_or_else(|err| {
        eprintln!("🔥 Failed to initialize S3 client: {:?}", err);
        std::process::exit(1);
    });


    // Wrap the pool in an Arc to enable shared ownership
    let shared_pool = pool;


    let subscription_repository = SubscriptionRepositoryImpl::new(shared_pool.clone());
    let subscription_type_repository = SubscriptionTypeRepositoryImpl::new(shared_pool.clone());
    let role_repository = RoleRepositoryImpl::new(shared_pool.clone());
    let province_repository = ProvinceRepositoryImpl::new(shared_pool.clone());
    let city_repository = CityRepositoryImpl::new(shared_pool.clone());
    let school_repository = SchoolRepositoryImpl::new(shared_pool.clone());
    let user_repository = UserRepositoryImpl::new(shared_pool.clone());
    let db_transaction_repository = DbTransactionRepositoryImpl::new(shared_pool.clone());

    let subscription_usecase = SubscriptionUseCaseImpl::new(subscription_repository.clone(), subscription_type_repository.clone());
    let subscription_type_usecase = SubscriptionTypeUseCaseImpl::new(subscription_type_repository.clone(), subscription_repository.clone());
    let role_usecase = RoleUseCaseImpl::new(role_repository.clone());
    let province_usecase = ProvinceUseCaseImpl::new(province_repository.clone());
    let city_usecase = CityUseCaseImpl::new(city_repository.clone(), province_repository.clone());
    let school_usecase = SchoolUseCaseImpl::new(school_repository.clone(), s3_client.clone());
    let user_usecase = UserUseCaseImpl::new(user_repository.clone(), role_repository.clone(), school_repository.clone());
    let auth_usecase = AuthUseCaseImpl::new(user_repository.clone(), role_repository.clone(), school_repository.clone(), db_transaction_repository.clone());

    let subscription_handler = SubscriptionHandlerImpl::new(subscription_usecase);
    let subscription_type_handler = SubscriptionTypeHandlerImpl::new(subscription_type_usecase);
    let role_handler = RoleHandlerImpl::new(role_usecase);
    let province_handler = ProvinceHandlerImpl::new(province_usecase);
    let city_handler = CityHandlerImpl::new(city_usecase);
    let school_handler = SchoolHandlerImpl::new(school_usecase);
    let user_handler = UserHandlerImpl::new(user_usecase);
    let auth_handler = AuthHandlerImpl::new(auth_usecase);

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
            .configure(|cfg| subscription_type_router(cfg, subscription_type_handler.clone()))
            .configure(|cfg| role_router(cfg, role_handler.clone()))
            .configure(|cfg| province_router(cfg, province_handler.clone()))
            .configure(|cfg| city_router(cfg, city_handler.clone()))
            .configure(|cfg| school_router(cfg, school_handler.clone()))
            .configure(|cfg| user_router(cfg, user_handler.clone()))
            .configure(|cfg| auth_router(cfg, auth_handler.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}