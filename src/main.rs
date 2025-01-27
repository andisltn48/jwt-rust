mod routes;
mod models;
mod security;


use std::env;
use actix_web::{middleware::Logger, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

use routes::{health_route, config::config};

pub struct AppState {
    db_pool: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
        Ok(db_pool) => {
            println!("✅Connection to database succesfully");
            db_pool
        },
        Err(e) => {
            println!("🔴 Connection to database error: {}", e);
            std::process::exit(1);
        }
    };

    println!("🚀 Starting server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db_pool: db_pool.clone() }))
            .service(health_route::health_checker_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
