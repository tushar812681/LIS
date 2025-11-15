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

    info!("Starting Compliance Service on {}:{}", config.host, config.port);

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

    // Build GraphQL schema
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db_pool.clone())
    .finish();

    info!("GraphQL schema built");

    // Start HTTP server
    let bind_address = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
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
            .service(web::resource("/ready").route(web::get().to(ready_check)))
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
        "service": "compliance-service",
        "version": env!("CARGO_PKG_VERSION"),
    })))
}

async fn ready_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
    })))
}
