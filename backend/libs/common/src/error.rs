use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // Database Errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("MongoDB error: {0}")]
    MongoDb(#[from] mongodb::error::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    // Authentication & Authorization
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    // Validation Errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    // Business Logic Errors
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    // Sample Management
    #[error("Sample not found")]
    SampleNotFound,

    #[error("Sample already rejected")]
    SampleAlreadyRejected,

    #[error("Insufficient sample volume")]
    InsufficientSampleVolume,

    #[error("Invalid sample status: {0}")]
    InvalidSampleStatus(String),

    #[error("Invalid sample quality: {0}")]
    InvalidSampleQuality(String),

    #[error("Insufficient volume: {0}")]
    InsufficientVolume(String),

    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),

    // Order Management
    #[error("Order not found")]
    OrderNotFound,

    #[error("Cannot modify order in current status")]
    CannotModifyOrder,

    #[error("Order already cancelled")]
    OrderAlreadyCancelled,

    // Result Management
    #[error("Result not found")]
    ResultNotFound,

    #[error("Cannot amend delivered result")]
    CannotAmendDeliveredResult,

    #[error("Critical value detected")]
    CriticalValueDetected,

    // Equipment Management
    #[error("Equipment not found")]
    EquipmentNotFound,

    #[error("Equipment offline")]
    EquipmentOffline,

    #[error("No suitable equipment available")]
    NoSuitableEquipment,

    // Payment & Billing
    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid payment method")]
    InvalidPaymentMethod,

    // External Service Errors
    #[error("Kafka error: {0}")]
    Kafka(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("UIDAI API error: {0}")]
    UidaiError(String),

    #[error("ABDM API error: {0}")]
    AbdmError(String),

    #[error("WhatsApp API error: {0}")]
    WhatsAppError(String),

    #[error("Payment gateway error: {0}")]
    PaymentGatewayError(String),

    // General Errors
    #[error("Internal server error")]
    InternalServerError,

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            // 400 Bad Request
            Self::Validation(_)
            | Self::InvalidInput(_)
            | Self::InvalidState(_)
            | Self::BusinessRuleViolation(_)
            | Self::CannotModifyOrder
            | Self::CannotAmendDeliveredResult => 400,

            // 401 Unauthorized
            Self::AuthenticationFailed(_)
            | Self::Unauthorized
            | Self::InvalidToken
            | Self::TokenExpired => 401,

            // 403 Forbidden
            Self::InsufficientPermissions => 403,

            // 404 Not Found
            Self::NotFound(_)
            | Self::SampleNotFound
            | Self::OrderNotFound
            | Self::ResultNotFound
            | Self::EquipmentNotFound => 404,

            // 409 Conflict
            Self::DuplicateEntry(_)
            | Self::AlreadyExists(_)
            | Self::SampleAlreadyRejected
            | Self::OrderAlreadyCancelled => 409,

            // 422 Unprocessable Entity
            Self::InsufficientSampleVolume
            | Self::InsufficientBalance
            | Self::InvalidPaymentMethod => 422,

            // 503 Service Unavailable
            Self::EquipmentOffline
            | Self::NoSuitableEquipment
            | Self::ExternalService(_)
            | Self::UidaiError(_)
            | Self::AbdmError(_)
            | Self::WhatsAppError(_)
            | Self::PaymentGatewayError(_) => 503,

            // 500 Internal Server Error
            _ => 500,
        }
    }

    pub fn error_code(&self) -> &str {
        match self {
            Self::Database(_) => "DATABASE_ERROR",
            Self::MongoDb(_) => "MONGODB_ERROR",
            Self::Redis(_) => "REDIS_ERROR",
            Self::AuthenticationFailed(_) => "AUTHENTICATION_FAILED",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::InsufficientPermissions => "INSUFFICIENT_PERMISSIONS",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::DuplicateEntry(_) => "DUPLICATE_ENTRY",
            Self::NotFound(_) => "NOT_FOUND",
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::InvalidState(_) => "INVALID_STATE",
            Self::BusinessRuleViolation(_) => "BUSINESS_RULE_VIOLATION",
            Self::SampleNotFound => "SAMPLE_NOT_FOUND",
            Self::SampleAlreadyRejected => "SAMPLE_ALREADY_REJECTED",
            Self::InsufficientSampleVolume => "INSUFFICIENT_SAMPLE_VOLUME",
            Self::InvalidSampleStatus(_) => "INVALID_SAMPLE_STATUS",
            Self::InvalidSampleQuality(_) => "INVALID_SAMPLE_QUALITY",
            Self::InsufficientVolume(_) => "INSUFFICIENT_VOLUME",
            Self::InvalidStatusTransition(_) => "INVALID_STATUS_TRANSITION",
            Self::OrderNotFound => "ORDER_NOT_FOUND",
            Self::CannotModifyOrder => "CANNOT_MODIFY_ORDER",
            Self::OrderAlreadyCancelled => "ORDER_ALREADY_CANCELLED",
            Self::ResultNotFound => "RESULT_NOT_FOUND",
            Self::CannotAmendDeliveredResult => "CANNOT_AMEND_DELIVERED_RESULT",
            Self::CriticalValueDetected => "CRITICAL_VALUE_DETECTED",
            Self::EquipmentNotFound => "EQUIPMENT_NOT_FOUND",
            Self::EquipmentOffline => "EQUIPMENT_OFFLINE",
            Self::NoSuitableEquipment => "NO_SUITABLE_EQUIPMENT",
            Self::PaymentFailed(_) => "PAYMENT_FAILED",
            Self::InsufficientBalance => "INSUFFICIENT_BALANCE",
            Self::InvalidPaymentMethod => "INVALID_PAYMENT_METHOD",
            Self::Kafka(_) => "KAFKA_ERROR",
            Self::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            Self::UidaiError(_) => "UIDAI_ERROR",
            Self::AbdmError(_) => "ABDM_ERROR",
            Self::WhatsAppError(_) => "WHATSAPP_ERROR",
            Self::PaymentGatewayError(_) => "PAYMENT_GATEWAY_ERROR",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
            Self::Configuration(_) => "CONFIGURATION_ERROR",
            Self::Serialization(_) => "SERIALIZATION_ERROR",
            Self::Io(_) => "IO_ERROR",
            Self::Custom(_) => "CUSTOM_ERROR",
        }
    }
}

// GraphQL Error Extensions
use async_graphql::ErrorExtensions;

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
            .extend_with(|_err, e| {
                e.set("code", self.error_code());
                e.set("status", self.status_code());
            })
    }
}
