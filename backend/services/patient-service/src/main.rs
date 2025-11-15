mod api;
mod config;
mod domain;
mod repository;
mod service;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::api::{MutationRoot, QueryRoot};
use crate::config::Config;

pub type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("Failed to load configuration");

    info!("Starting Patient Service on {}:{}", config.host, config.port);

    // Database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    info!("Database migrations completed");

    // Redis connection
    let redis_client = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    info!("Connected to Redis");

    // Build GraphQL schema
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db_pool.clone())
    .data(redis_client.clone())
    .finish();

    info!("GraphQL schema built");

    // Start HTTP server
    let bind_address = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql_playground)),
            )
            .service(web::resource("/health").route(web::get().to(health_check)))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn graphql(
    schema: web::Data<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    let html = async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    );
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "patient-service",
        "version": env!("CARGO_PKG_VERSION"),
    })))
}
