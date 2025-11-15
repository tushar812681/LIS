use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "notification_channel", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationChannel {
    Email,
    Sms,
    Whatsapp,
    Push,
    InApp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "notification_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationStatus {
    Pending,
    Queued,
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
    Bounced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "notification_priority", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "template_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TemplateType {
    AppointmentReminder,
    TestResultReady,
    InvoiceGenerated,
    PaymentReceived,
    ReportDelivery,
    QcAlert,
    EquipmentMaintenance,
    StockAlert,
    Custom,
}

// ============================================================================
// Notification Template Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub template_name: String,
    pub template_code: String,
    pub template_type: TemplateType,
    pub description: Option<String>,

    pub email_subject: Option<String>,
    pub email_body: Option<String>,
    pub email_html_body: Option<String>,

    pub sms_content: Option<String>,
    pub sms_length: Option<i32>,

    pub whatsapp_content: Option<String>,
    pub whatsapp_template_id: Option<String>,

    pub push_title: Option<String>,
    pub push_body: Option<String>,

    #[sqlx(json)]
    pub variables: Option<serde_json::Value>,

    #[sqlx(skip)]
    pub supported_channels: Vec<NotificationChannel>,
    pub default_channel: Option<NotificationChannel>,
    pub is_active: Option<bool>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Notification Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub template_id: Option<Uuid>,

    pub recipient_id: Option<Uuid>,
    pub recipient_type: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_contact: String,

    pub notification_channel: NotificationChannel,
    pub notification_priority: Option<NotificationPriority>,

    pub subject: Option<String>,
    pub content: String,
    pub html_content: Option<String>,

    #[sqlx(json)]
    pub template_data: Option<serde_json::Value>,

    pub scheduled_at: Option<NaiveDateTime>,
    pub sent_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,
    pub read_at: Option<NaiveDateTime>,

    pub notification_status: Option<NotificationStatus>,
    pub status_message: Option<String>,

    pub provider_name: Option<String>,
    pub provider_message_id: Option<String>,
    #[sqlx(json)]
    pub provider_response: Option<serde_json::Value>,

    pub retry_count: Option<i32>,
    pub max_retries: Option<i32>,
    pub last_retry_at: Option<NaiveDateTime>,

    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,

    #[sqlx(json)]
    pub metadata: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,

    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

impl Notification {
    pub fn is_delivered(&self) -> bool {
        matches!(
            self.notification_status,
            Some(NotificationStatus::Delivered) | Some(NotificationStatus::Read)
        )
    }

    pub fn can_retry(&self) -> bool {
        let retry_count = self.retry_count.unwrap_or(0);
        let max_retries = self.max_retries.unwrap_or(3);
        retry_count < max_retries && self.notification_status == Some(NotificationStatus::Failed)
    }
}

// ============================================================================
// Notification Preference Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,

    pub email_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub whatsapp_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,

    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub push_token: Option<String>,

    #[sqlx(json)]
    pub template_preferences: Option<serde_json::Value>,

    pub quiet_hours_enabled: Option<bool>,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,

    pub max_notifications_per_day: Option<i32>,
    pub max_notifications_per_hour: Option<i32>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl NotificationPreference {
    pub fn is_channel_enabled(&self, channel: NotificationChannel) -> bool {
        match channel {
            NotificationChannel::Email => self.email_enabled.unwrap_or(true),
            NotificationChannel::Sms => self.sms_enabled.unwrap_or(true),
            NotificationChannel::Whatsapp => self.whatsapp_enabled.unwrap_or(true),
            NotificationChannel::Push => self.push_enabled.unwrap_or(true),
            NotificationChannel::InApp => self.in_app_enabled.unwrap_or(true),
        }
    }

    pub fn is_in_quiet_hours(&self) -> bool {
        if !self.quiet_hours_enabled.unwrap_or(false) {
            return false;
        }

        if let (Some(start), Some(end)) = (self.quiet_hours_start, self.quiet_hours_end) {
            let now = chrono::Local::now().time();
            if start < end {
                now >= start && now <= end
            } else {
                // Quiet hours cross midnight
                now >= start || now <= end
            }
        } else {
            false
        }
    }
}

// ============================================================================
// Notification Queue Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct NotificationQueue {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub queue_name: String,
    pub batch_id: Option<Uuid>,

    pub notification_id: Option<Uuid>,

    pub priority: Option<i32>,
    pub scheduled_for: NaiveDateTime,

    pub is_processed: Option<bool>,
    pub processed_at: Option<NaiveDateTime>,
    pub processing_started_at: Option<NaiveDateTime>,
    pub processing_by: Option<String>,

    pub error_count: Option<i32>,
    pub last_error: Option<String>,

    pub created_at: NaiveDateTime,
}

// ============================================================================
// Notification Log Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct NotificationLog {
    pub id: Uuid,
    pub notification_id: Uuid,

    pub event_type: String,
    pub event_timestamp: NaiveDateTime,

    #[sqlx(json)]
    pub event_data: Option<serde_json::Value>,
    pub event_message: Option<String>,

    pub event_source: Option<String>,
}

// ============================================================================
// Provider Configuration Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ProviderConfiguration {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub provider_name: String,
    pub provider_type: NotificationChannel,

    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub endpoint_url: Option<String>,
    pub from_number: Option<String>,
    pub from_email: Option<String>,
    #[sqlx(json)]
    pub configuration: Option<serde_json::Value>,

    pub is_active: Option<bool>,
    pub is_default: Option<bool>,

    pub daily_limit: Option<i32>,
    pub monthly_limit: Option<i32>,
    #[sqlx(skip)]
    #[serde(skip)]
    pub rate_limit_per_second: Option<rust_decimal::Decimal>,

    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(Debug, Clone, InputObject)]
pub struct CreateNotificationTemplateInput {
    pub organization_id: Uuid,
    pub template_name: String,
    pub template_code: String,
    pub template_type: TemplateType,
    pub description: Option<String>,
    pub email_subject: Option<String>,
    pub email_body: Option<String>,
    pub sms_content: Option<String>,
    pub whatsapp_content: Option<String>,
    pub push_title: Option<String>,
    pub push_body: Option<String>,
    pub variables: Option<String>, // JSON string
    pub supported_channels: Vec<NotificationChannel>,
    pub default_channel: Option<NotificationChannel>,
}

#[derive(Debug, Clone, InputObject)]
pub struct SendNotificationInput {
    pub organization_id: Uuid,
    pub template_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub recipient_type: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_contact: String,
    pub notification_channel: NotificationChannel,
    pub notification_priority: Option<NotificationPriority>,
    pub subject: Option<String>,
    pub content: String,
    pub template_data: Option<String>, // JSON string
    pub scheduled_at: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateNotificationPreferenceInput {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub email_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub whatsapp_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub quiet_hours_enabled: Option<bool>,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateProviderConfigInput {
    pub organization_id: Uuid,
    pub provider_name: String,
    pub provider_type: NotificationChannel,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub endpoint_url: Option<String>,
    pub from_number: Option<String>,
    pub from_email: Option<String>,
    pub configuration: Option<String>, // JSON string
    pub is_default: Option<bool>,
}

// ============================================================================
// Filter Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct NotificationTemplateFilter {
    pub organization_id: Option<Uuid>,
    pub template_type: Option<TemplateType>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NotificationFilter {
    pub organization_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub notification_channel: Option<NotificationChannel>,
    pub notification_status: Option<NotificationStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct NotificationQueueFilter {
    pub organization_id: Option<Uuid>,
    pub is_processed: Option<bool>,
}
