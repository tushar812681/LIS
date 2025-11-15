use actix_cors::Cors;
use actix_web::{get, guard, middleware, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde_json::json;
use std::env;

mod patient_client;
mod schema;

use schema::*;

/// Health check endpoint
#[get("/health")]
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "api-gateway",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

/// Ready check endpoint
#[get("/ready")]
async fn ready() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ready"
    })))
}

/// GraphQL endpoint
async fn graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL Playground UI
async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    tracing::info!("Starting API Gateway...");

    // Configuration
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("{}:{}", host, port);

    // Service URLs
    let patient_service_url =
        env::var("PATIENT_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let order_service_url =
        env::var("ORDER_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());
    let sample_service_url =
        env::var("SAMPLE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8083".to_string());
    let result_service_url =
        env::var("RESULT_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8084".to_string());
    let user_service_url =
        env::var("USER_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let organization_service_url =
        env::var("ORGANIZATION_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8094".to_string());

    tracing::info!("Configuration loaded:");
    tracing::info!("  Gateway: {}", bind_address);
    tracing::info!("  Patient Service: {}", patient_service_url);
    tracing::info!("  Order Service: {}", order_service_url);
    tracing::info!("  Sample Service: {}", sample_service_url);
    tracing::info!("  Result Service: {}", result_service_url);
    tracing::info!("  User Service: {}", user_service_url);
    tracing::info!("  Organization Service: {}", organization_service_url);

    // Create service clients
    let patient_client = patient_client::PatientClient::new(patient_service_url);

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(patient_client)
        .finish();

    tracing::info!("GraphQL schema built successfully");
    tracing::info!("Starting HTTP server on {}", bind_address);
    tracing::info!("GraphQL endpoint: http://{}/graphql", bind_address);
    tracing::info!("GraphQL playground: http://{}/graphql (GET)", bind_address);
    tracing::info!("Health check: http://{}/health", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3001")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(schema.clone()))
            .service(health)
            .service(ready)
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
