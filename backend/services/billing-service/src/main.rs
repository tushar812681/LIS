use actix_web::{web, App, HttpServer, middleware, HttpResponse, guard};
use async_graphql::{Schema, EmptySubscription, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;

mod domain;
mod repository;
mod service;
mod api;
mod config;

use repository::*;
use service::BillingService;
use api::{QueryRoot, MutationRoot};
use config::Config;

type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql(
    schema: web::Data<BillingSchema>,
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
        "service": "billing-service",
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

    tracing::info!("Starting Billing Service...");

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config from environment: {}. Using defaults.", e);
        Config::default()
    });

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
    let invoice_repo = InvoiceRepository::new(pool.clone());
    let payment_repo = PaymentRepository::new(pool.clone());
    let insurance_company_repo = InsuranceCompanyRepository::new(pool.clone());
    let insurance_claim_repo = InsuranceClaimRepository::new(pool.clone());
    let credit_note_repo = CreditNoteRepository::new(pool.clone());
    let discount_scheme_repo = DiscountSchemeRepository::new(pool.clone());

    // Create service
    let billing_service = BillingService::new(
        invoice_repo,
        payment_repo,
        insurance_company_repo,
        insurance_claim_repo,
        credit_note_repo,
        discount_scheme_repo,
    );

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(billing_service)
        .finish();

    tracing::info!("GraphQL schema built successfully");
    tracing::info!("  Queries: invoice, invoiceByNumber, invoices, patientInvoices, orderInvoices, invoiceItems, payment, payments, invoicePayments, insuranceCompany, insuranceCompanies, insuranceClaim, insuranceClaims, creditNote, invoiceCreditNotes, discountScheme, discountSchemes, applicableDiscountSchemes");
    tracing::info!("  Mutations: createInvoice, cancelInvoice, updateInvoiceStatus, recordPayment, reconcilePayment, createInsuranceCompany, createInsuranceClaim, updateClaimStatus, createCreditNote, createDiscountScheme");

    // Start HTTP server
    let bind_addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting HTTP server on {}", bind_addr);
    tracing::info!("GraphQL endpoint: http://{}/graphql", bind_addr);
    tracing::info!("GraphiQL playground: http://{}/graphql (GET)", bind_addr);
    tracing::info!("Health check: http://{}/health", bind_addr);
    tracing::info!("Ready check: http://{}/ready", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql)
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(graphql_playground)
            )
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/ready").route(web::get().to(ready_check)))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
