use actix_web::{web, HttpServer, App, HttpResponse};
use actix_web::middleware::{Logger, from_fn};
use sqlx::MySqlPool;
use dotenvy::dotenv;
use std::env;
use crate::utils::auth_middleware;
use crate::utils::handlers::{register, login, protected, create_user, fetch_all_users, update_user, delete_user};
use std::time::Duration;

pub async fn run_server() -> MySqlPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    // Error handling when DB connection fails
    let pool = MySqlPool::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database: {}", db_url));

    let actix_pool = pool.clone();

    // Start Actix server in a separate task
    tokio::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(actix_pool.clone()))
                .wrap(Logger::default())
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(register))
                        .route("/login", web::post().to(login)),
                )
                .service(
                    web::scope("/api")
                        .wrap(from_fn(auth_middleware::check_auth_middleware))
                        .route("/protected", web::get().to(protected)),
                )
                .service(
                    web::scope("/api/users")
                        .wrap(from_fn(auth_middleware::check_auth_middleware))
                        .route("/create_user", web::post().to(create_user))
                        .route("/fetch_all_users", web::get().to(fetch_all_users))
                        .route("/update_user", web::put().to(update_user))
                        .route("/delete_user/{id}", web::delete().to(delete_user)),
                )
        })
        .bind(("0.0.0.0", 8080))  // Bind to all IP addresses for wider accessibility
        .expect("Failed to bind server")
        .shutdown_timeout(Duration::from_secs(10))  // Graceful shutdown in 10 seconds
        .run()
        .await
        .expect("Server run failed");
    });

    pool
}
