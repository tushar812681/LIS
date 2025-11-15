use actix_web::{web, App, HttpServer, middleware, HttpResponse, guard};
use actix_cors::Cors;
use async_graphql::{Schema, EmptySubscription, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;

mod domain;
mod repository;
mod service;
mod api;
mod config;
mod organization_client;

use repository::*;
use service::UserService;
use api::{QueryRoot, MutationRoot};
use config::Config;
use organization_client::OrganizationClient;

type UserSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql(
    schema: web::Data<UserSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn ready_check(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "database": "connected",
        })),
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not ready",
            "database": "disconnected",
            "error": e.to_string(),
        })),
    }
}

fn mask_password(url: &str) -> String {
    if let Some(start) = url.find("://") {
        if let Some(end) = url[start + 3..].find('@') {
            let before = &url[..start + 3];
            let after = &url[start + 3 + end..];
            if let Some(colon) = url[start + 3..start + 3 + end].find(':') {
                let user = &url[start + 3..start + 3 + colon];
                return format!("{}{}:****{}", before, user, after);
            }
        }
    }
    url.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .init();

    tracing::info!("Starting User Service...");

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config from environment: {}. Using defaults.", e);
        Config::default()
    });

    // Set JWT secret as environment variable for service layer
    std::env::set_var("JWT_SECRET", &config.jwt_secret);

    tracing::info!("Configuration loaded:");
    tracing::info!("  Database: {}", mask_password(&config.database_url));
    tracing::info!("  Server: {}:{}", config.host, config.port);
    tracing::info!("  Max DB connections: {}", config.database_max_connections);
    tracing::info!("  Caching enabled: {}", config.enable_caching);
    tracing::info!("  Events enabled: {}", config.enable_events);

    // Create database pool
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connected successfully");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations completed successfully");

    // Create repositories
    let user_repo = UserRepository::new(pool.clone());
    let role_repo = RoleRepository::new(pool.clone());
    let permission_repo = PermissionRepository::new(pool.clone());
    let user_role_repo = UserRoleRepository::new(pool.clone());
    let session_repo = SessionRepository::new(pool.clone());
    let activity_repo = ActivityLogRepository::new(pool.clone());

    // Create organization client
    // Default to port 8094 for organization-service
    let org_service_url = std::env::var("ORGANIZATION_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8094".to_string());
    let org_client = OrganizationClient::new(org_service_url.clone());
    tracing::info!("Organization service client configured: {}", org_service_url);

    // Create service
    let user_service = UserService::new(
        user_repo,
        role_repo,
        permission_repo,
        user_role_repo,
        session_repo,
        activity_repo,
        org_client,
    );

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(user_service)
        .finish();

    tracing::info!("GraphQL schema built successfully");
    tracing::info!("  Queries: me, user, userByEmail, searchUsers, roles, permissions, userRoles, userPermissions");
    tracing::info!("  Mutations: register, login, logout, changePassword, requestPasswordReset, updateUserStatus, createRole, assignRole, removeRole, verifyEmail");

    // Start HTTP server
    let bind_addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting HTTP server on {}", bind_addr);
    tracing::info!("GraphQL endpoint: http://{}/graphql", bind_addr);
    tracing::info!("GraphiQL playground: http://{}/graphql (GET)", bind_addr);
    tracing::info!("Health check: http://{}/health", bind_addr);
    tracing::info!("Ready check: http://{}/ready", bind_addr);

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
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql_playground))
            )
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/ready").route(web::get().to(ready_check)))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
