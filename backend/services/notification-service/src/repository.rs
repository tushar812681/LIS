use crate::domain::*;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Local};

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Database(String),
    InvalidInput(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// Notification Template Repository
// ============================================================================

#[derive(Clone)]
pub struct NotificationTemplateRepository {
    pool: PgPool,
}

impl NotificationTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateNotificationTemplateInput, created_by: Uuid) -> Result<NotificationTemplate> {
        let variables: Option<serde_json::Value> = input.variables
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid variables JSON: {}", e)))?;

        // Convert supported_channels to JSON for storage
        let supported_channels_json = serde_json::to_value(&input.supported_channels)
            .map_err(|e| Error::InvalidInput(format!("Invalid supported_channels: {}", e)))?;

        let mut template = sqlx::query_as::<_, NotificationTemplate>(
            r#"
            INSERT INTO notification_template (
                organization_id, template_name, template_code, template_type, description,
                email_subject, email_body, sms_content, whatsapp_content,
                push_title, push_body, variables, supported_channels, default_channel,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.template_name)
        .bind(input.template_code)
        .bind(input.template_type)
        .bind(input.description)
        .bind(input.email_subject)
        .bind(input.email_body)
        .bind(input.sms_content)
        .bind(input.whatsapp_content)
        .bind(input.push_title)
        .bind(input.push_body)
        .bind(variables)
        .bind(supported_channels_json)
        .bind(input.default_channel)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        // Restore the supported_channels field
        template.supported_channels = input.supported_channels;

        Ok(template)
    }

    pub async fn get_by_id(&self, template_id: Uuid) -> Result<NotificationTemplate> {
        let template = sqlx::query_as::<_, NotificationTemplate>(
            "SELECT * FROM notification_template WHERE id = $1 AND is_deleted = false"
        )
        .bind(template_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Template with ID {} not found", template_id)))?;

        Ok(template)
    }

    pub async fn get_by_code(&self, organization_id: Uuid, template_code: &str) -> Result<NotificationTemplate> {
        let template = sqlx::query_as::<_, NotificationTemplate>(
            "SELECT * FROM notification_template WHERE organization_id = $1 AND template_code = $2 AND is_deleted = false"
        )
        .bind(organization_id)
        .bind(template_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Template '{}' not found", template_code)))?;

        Ok(template)
    }

    pub async fn list(&self, filter: Option<NotificationTemplateFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<NotificationTemplate>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM notification_template WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(template_type) = f.template_type {
                let type_str = match template_type {
                    TemplateType::AppointmentReminder => "APPOINTMENT_REMINDER",
                    TemplateType::TestResultReady => "TEST_RESULT_READY",
                    TemplateType::InvoiceGenerated => "INVOICE_GENERATED",
                    TemplateType::PaymentReceived => "PAYMENT_RECEIVED",
                    TemplateType::ReportDelivery => "REPORT_DELIVERY",
                    TemplateType::QcAlert => "QC_ALERT",
                    TemplateType::EquipmentMaintenance => "EQUIPMENT_MAINTENANCE",
                    TemplateType::StockAlert => "STOCK_ALERT",
                    TemplateType::Custom => "CUSTOM",
                };
                query.push_str(&format!(" AND template_type = '{}'", type_str));
            }
            if let Some(is_active) = f.is_active {
                query.push_str(&format!(" AND is_active = {}", is_active));
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let templates = sqlx::query_as::<_, NotificationTemplate>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(templates)
    }
}

// ============================================================================
// Notification Repository
// ============================================================================

#[derive(Clone)]
pub struct NotificationRepository {
    pool: PgPool,
}

impl NotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: SendNotificationInput, created_by: Option<Uuid>) -> Result<Notification> {
        let template_data: Option<serde_json::Value> = input.template_data
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid template_data JSON: {}", e)))?;

        let scheduled_at = input.scheduled_at
            .as_ref()
            .map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid scheduled_at format: {}", e)))?;

        let notification = sqlx::query_as::<_, Notification>(
            r#"
            INSERT INTO notification (
                organization_id, template_id, recipient_id, recipient_type,
                recipient_name, recipient_contact, notification_channel,
                notification_priority, subject, content, template_data,
                scheduled_at, reference_type, reference_id, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.template_id)
        .bind(input.recipient_id)
        .bind(input.recipient_type)
        .bind(input.recipient_name)
        .bind(input.recipient_contact)
        .bind(input.notification_channel)
        .bind(input.notification_priority.unwrap_or(NotificationPriority::Normal))
        .bind(input.subject)
        .bind(input.content)
        .bind(template_data)
        .bind(scheduled_at)
        .bind(input.reference_type)
        .bind(input.reference_id)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(notification)
    }

    pub async fn get_by_id(&self, notification_id: Uuid) -> Result<Notification> {
        let notification = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notification WHERE id = $1 AND is_deleted = false"
        )
        .bind(notification_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Notification with ID {} not found", notification_id)))?;

        Ok(notification)
    }

    pub async fn list(&self, filter: Option<NotificationFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Notification>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM notification WHERE is_deleted = false");

        if let Some(f) = &filter {
            if let Some(org_id) = f.organization_id {
                query.push_str(&format!(" AND organization_id = '{}'", org_id));
            }
            if let Some(recipient_id) = f.recipient_id {
                query.push_str(&format!(" AND recipient_id = '{}'", recipient_id));
            }
            if let Some(channel) = f.notification_channel {
                let channel_str = match channel {
                    NotificationChannel::Email => "EMAIL",
                    NotificationChannel::Sms => "SMS",
                    NotificationChannel::Whatsapp => "WHATSAPP",
                    NotificationChannel::Push => "PUSH",
                    NotificationChannel::InApp => "IN_APP",
                };
                query.push_str(&format!(" AND notification_channel = '{}'", channel_str));
            }
            if let Some(status) = f.notification_status {
                let status_str = match status {
                    NotificationStatus::Pending => "PENDING",
                    NotificationStatus::Queued => "QUEUED",
                    NotificationStatus::Sending => "SENDING",
                    NotificationStatus::Sent => "SENT",
                    NotificationStatus::Delivered => "DELIVERED",
                    NotificationStatus::Read => "READ",
                    NotificationStatus::Failed => "FAILED",
                    NotificationStatus::Bounced => "BOUNCED",
                };
                query.push_str(&format!(" AND notification_status = '{}'", status_str));
            }
            if let Some(from_date) = f.from_date {
                query.push_str(&format!(" AND created_at >= '{}'", from_date));
            }
            if let Some(to_date) = f.to_date {
                query.push_str(&format!(" AND created_at <= '{}'", to_date));
            }
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", page_size, offset));

        let notifications = sqlx::query_as::<_, Notification>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(notifications)
    }

    pub async fn update_status(&self, notification_id: Uuid, status: NotificationStatus, status_message: Option<String>) -> Result<Notification> {
        let notification = sqlx::query_as::<_, Notification>(
            r#"
            UPDATE notification
            SET notification_status = $2,
                status_message = $3,
                sent_at = CASE WHEN $2 IN ('SENT', 'DELIVERED', 'READ') THEN COALESCE(sent_at, CURRENT_TIMESTAMP) ELSE sent_at END,
                delivered_at = CASE WHEN $2 IN ('DELIVERED', 'READ') THEN COALESCE(delivered_at, CURRENT_TIMESTAMP) ELSE delivered_at END,
                read_at = CASE WHEN $2 = 'READ' THEN CURRENT_TIMESTAMP ELSE read_at END
            WHERE id = $1 AND is_deleted = false
            RETURNING *
            "#
        )
        .bind(notification_id)
        .bind(status)
        .bind(status_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(notification)
    }

    pub async fn increment_retry(&self, notification_id: Uuid) -> Result<Notification> {
        let notification = sqlx::query_as::<_, Notification>(
            "UPDATE notification SET retry_count = retry_count + 1, last_retry_at = CURRENT_TIMESTAMP WHERE id = $1 AND is_deleted = false RETURNING *"
        )
        .bind(notification_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(notification)
    }

    pub async fn get_pending(&self, limit: i64) -> Result<Vec<Notification>> {
        let notifications = sqlx::query_as::<_, Notification>(
            r#"
            SELECT * FROM notification
            WHERE is_deleted = false
            AND notification_status = 'PENDING'
            AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP)
            ORDER BY notification_priority DESC, created_at ASC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(notifications)
    }
}

// ============================================================================
// Notification Preference Repository
// ============================================================================

#[derive(Clone)]
pub struct NotificationPreferenceRepository {
    pool: PgPool,
}

impl NotificationPreferenceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, input: UpdateNotificationPreferenceInput) -> Result<NotificationPreference> {
        let quiet_start = input.quiet_hours_start
            .as_ref()
            .map(|s| NaiveTime::parse_from_str(s, "%H:%M:%S"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid quiet_hours_start: {}", e)))?;

        let quiet_end = input.quiet_hours_end
            .as_ref()
            .map(|s| NaiveTime::parse_from_str(s, "%H:%M:%S"))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid quiet_hours_end: {}", e)))?;

        let preference = sqlx::query_as::<_, NotificationPreference>(
            r#"
            INSERT INTO notification_preference (
                organization_id, user_id, email_enabled, sms_enabled, whatsapp_enabled,
                push_enabled, email_address, phone_number, whatsapp_number,
                quiet_hours_enabled, quiet_hours_start, quiet_hours_end
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (organization_id, user_id)
            DO UPDATE SET
                email_enabled = COALESCE($3, notification_preference.email_enabled),
                sms_enabled = COALESCE($4, notification_preference.sms_enabled),
                whatsapp_enabled = COALESCE($5, notification_preference.whatsapp_enabled),
                push_enabled = COALESCE($6, notification_preference.push_enabled),
                email_address = COALESCE($7, notification_preference.email_address),
                phone_number = COALESCE($8, notification_preference.phone_number),
                whatsapp_number = COALESCE($9, notification_preference.whatsapp_number),
                quiet_hours_enabled = COALESCE($10, notification_preference.quiet_hours_enabled),
                quiet_hours_start = COALESCE($11, notification_preference.quiet_hours_start),
                quiet_hours_end = COALESCE($12, notification_preference.quiet_hours_end),
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.user_id)
        .bind(input.email_enabled)
        .bind(input.sms_enabled)
        .bind(input.whatsapp_enabled)
        .bind(input.push_enabled)
        .bind(input.email_address)
        .bind(input.phone_number)
        .bind(input.whatsapp_number)
        .bind(input.quiet_hours_enabled)
        .bind(quiet_start)
        .bind(quiet_end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(preference)
    }

    pub async fn get_by_user(&self, organization_id: Uuid, user_id: Uuid) -> Result<NotificationPreference> {
        let preference = sqlx::query_as::<_, NotificationPreference>(
            "SELECT * FROM notification_preference WHERE organization_id = $1 AND user_id = $2"
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Preference for user {} not found", user_id)))?;

        Ok(preference)
    }
}

// ============================================================================
// Notification Log Repository
// ============================================================================

#[derive(Clone)]
pub struct NotificationLogRepository {
    pool: PgPool,
}

impl NotificationLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log_event(&self, notification_id: Uuid, event_type: &str, event_message: Option<String>) -> Result<NotificationLog> {
        let log = sqlx::query_as::<_, NotificationLog>(
            "INSERT INTO notification_log (notification_id, event_type, event_message, event_source) VALUES ($1, $2, $3, 'SYSTEM') RETURNING *"
        )
        .bind(notification_id)
        .bind(event_type)
        .bind(event_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(log)
    }

    pub async fn get_by_notification(&self, notification_id: Uuid) -> Result<Vec<NotificationLog>> {
        let logs = sqlx::query_as::<_, NotificationLog>(
            "SELECT * FROM notification_log WHERE notification_id = $1 ORDER BY event_timestamp DESC"
        )
        .bind(notification_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(logs)
    }
}

// ============================================================================
// Provider Configuration Repository
// ============================================================================

#[derive(Clone)]
pub struct ProviderConfigurationRepository {
    pool: PgPool,
}

impl ProviderConfigurationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateProviderConfigInput, created_by: Uuid) -> Result<ProviderConfiguration> {
        let configuration: Option<serde_json::Value> = input.configuration
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|e| Error::InvalidInput(format!("Invalid configuration JSON: {}", e)))?;

        let config = sqlx::query_as::<_, ProviderConfiguration>(
            r#"
            INSERT INTO provider_configuration (
                organization_id, provider_name, provider_type, api_key, api_secret,
                endpoint_url, from_number, from_email, configuration, is_default, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(input.organization_id)
        .bind(input.provider_name)
        .bind(input.provider_type)
        .bind(input.api_key)
        .bind(input.api_secret)
        .bind(input.endpoint_url)
        .bind(input.from_number)
        .bind(input.from_email)
        .bind(configuration)
        .bind(input.is_default.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(config)
    }

    pub async fn get_default(&self, organization_id: Uuid, provider_type: NotificationChannel) -> Result<ProviderConfiguration> {
        let config = sqlx::query_as::<_, ProviderConfiguration>(
            "SELECT * FROM provider_configuration WHERE organization_id = $1 AND provider_type = $2 AND is_default = true AND is_active = true AND is_deleted = false LIMIT 1"
        )
        .bind(organization_id)
        .bind(provider_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .ok_or_else(|| Error::NotFound(format!("Default provider for {:?} not found", provider_type)))?;

        Ok(config)
    }
}
