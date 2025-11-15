use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod api;
mod config;
mod domain;
mod minio_client;
mod repository;
mod service;

use api::*;
use config::Config;
use minio_client::MinioClient;
use repository::FileRepository;
use service::FileService;

type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_playground() -> Result<HttpResponse> {
    let source = GraphiQLSource::build().endpoint("/graphql").finish();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

async fn graphql_handler(
    schema: web::Data<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "file-service",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    tracing::info!("Starting File Service on {}:{}", config.host, config.port);

    // Create database pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database migrations completed");

    // Create MinIO client
    let minio_client = MinioClient::new(
        &config.minio_endpoint,
        &config.minio_access_key,
        &config.minio_secret_key,
        config.minio_use_ssl,
        &config.minio_bucket_name,
    )
    .await
    .expect("Failed to create MinIO client");

    // Ensure bucket exists
    minio_client
        .ensure_bucket_exists()
        .await
        .expect("Failed to ensure bucket exists");

    tracing::info!("MinIO client initialized successfully");

    // Create repository
    let repository = FileRepository::new(pool.clone());

    // Create service
    let file_service = FileService::new(repository, minio_client, config.clone());

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(file_service)
        .finish();

    tracing::info!("GraphQL schema built successfully");

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
            .app_data(web::PayloadConfig::new(52428800)) // 50MB max payload
            .route("/health", web::get().to(health_check))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphiql", web::get().to(graphql_playground))
    })
    .bind(&bind_address)?
    .run()
    .await
}
