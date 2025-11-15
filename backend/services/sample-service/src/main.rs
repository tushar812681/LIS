use actix_web::{web, App, HttpServer, HttpResponse, middleware, guard};
use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, error};
use tracing_subscriber;

mod config;
mod domain;
mod repository;
mod service;
mod api;

use config::Config;
use repository::{SampleRepository, SampleAliquotRepository, SampleRoutingRepository};
use service::SampleService;
use api::{QueryRoot, MutationRoot};

type SampleSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Sample Service...");

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|_| {
        info!("Using default configuration");
        Config::default()
    });

    info!("Configuration loaded");
    info!("Server will listen on {}:{}", config.host, config.port);
    info!("Database: {}", mask_password(&config.database_url));

    // Create database connection pool
    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection established");

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Migrations completed successfully");

    // Create repositories
    let sample_repo = SampleRepository::new(pool.clone());
    let aliquot_repo = SampleAliquotRepository::new(pool.clone());
    let routing_repo = SampleRoutingRepository::new(pool.clone());

    // Create service
    let sample_service = SampleService::new(sample_repo, aliquot_repo, routing_repo);

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(sample_service)
        .finish();

    info!("GraphQL schema built");

    let host = config.host.clone();
    let port = config.port;

    // Start HTTP server
    info!("Starting HTTP server...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/graphql").guard(guard::Get()).to(graphql_playground))
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/ready").route(web::get().to(readiness_check)))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

async fn graphql(schema: web::Data<SampleSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/graphql")
                .title("Sample Service - GraphQL Playground")
                .finish()
        )
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "sample-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn readiness_check() -> HttpResponse {
    // TODO: Check database connection, cache, etc.
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "service": "sample-service",
    }))
}

fn mask_password(url: &str) -> String {
    if let Some(pos) = url.find("://") {
        let after_protocol = &url[pos + 3..];
        if let Some(at_pos) = after_protocol.find('@') {
            let before_at = &after_protocol[..at_pos];
            if let Some(colon_pos) = before_at.find(':') {
                let user = &before_at[..colon_pos];
                let after_at = &after_protocol[at_pos..];
                return format!("{}://{}:****{}", &url[..pos], user, after_at);
            }
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_password() {
        let url = "postgres://user:password@localhost:5432/db";
        let masked = mask_password(url);
        assert_eq!(masked, "postgres://user:****@localhost:5432/db");
    }
}
