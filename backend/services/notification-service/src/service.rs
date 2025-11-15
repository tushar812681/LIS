use crate::domain::*;
use crate::repository::*;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug)]
pub enum NotificationError {
    NotFound(String),
    ValidationError(String),
    DeliveryFailed(String),
    ProviderNotConfigured(String),
    QuietHoursActive,
    ChannelDisabled(String),
    DatabaseError(String),
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::DeliveryFailed(msg) => write!(f, "Delivery failed: {}", msg),
            Self::ProviderNotConfigured(msg) => write!(f, "Provider not configured: {}", msg),
            Self::QuietHoursActive => write!(f, "Notification blocked: quiet hours active"),
            Self::ChannelDisabled(msg) => write!(f, "Channel disabled: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for NotificationError {}

impl From<Error> for NotificationError {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound(msg) => NotificationError::NotFound(msg),
            Error::Database(msg) => NotificationError::DatabaseError(msg),
            Error::InvalidInput(msg) => NotificationError::ValidationError(msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, NotificationError>;

#[derive(Clone)]
pub struct NotificationService {
    template_repo: NotificationTemplateRepository,
    notification_repo: NotificationRepository,
    preference_repo: NotificationPreferenceRepository,
    log_repo: NotificationLogRepository,
    provider_repo: ProviderConfigurationRepository,
}

impl NotificationService {
    pub fn new(
        template_repo: NotificationTemplateRepository,
        notification_repo: NotificationRepository,
        preference_repo: NotificationPreferenceRepository,
        log_repo: NotificationLogRepository,
        provider_repo: ProviderConfigurationRepository,
    ) -> Self {
        Self {
            template_repo,
            notification_repo,
            preference_repo,
            log_repo,
            provider_repo,
        }
    }

    // ============================================================================
    // Template Operations
    // ============================================================================

    pub async fn create_template(&self, input: CreateNotificationTemplateInput, created_by: Uuid) -> Result<NotificationTemplate> {
        if input.template_name.is_empty() {
            return Err(NotificationError::ValidationError("Template name is required".to_string()));
        }

        if input.template_code.is_empty() {
            return Err(NotificationError::ValidationError("Template code is required".to_string()));
        }

        if input.supported_channels.is_empty() {
            return Err(NotificationError::ValidationError("At least one channel must be supported".to_string()));
        }

        let template = self.template_repo.create(input, created_by).await?;
        Ok(template)
    }

    pub async fn get_template(&self, template_id: Uuid) -> Result<NotificationTemplate> {
        let template = self.template_repo.get_by_id(template_id).await?;
        Ok(template)
    }

    pub async fn get_template_by_code(&self, organization_id: Uuid, template_code: &str) -> Result<NotificationTemplate> {
        let template = self.template_repo.get_by_code(organization_id, template_code).await?;
        Ok(template)
    }

    pub async fn list_templates(&self, filter: Option<NotificationTemplateFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<NotificationTemplate>> {
        let templates = self.template_repo.list(filter, page, page_size).await?;
        Ok(templates)
    }

    // ============================================================================
    // Notification Operations
    // ============================================================================

    pub async fn send_notification(&self, mut input: SendNotificationInput, created_by: Option<Uuid>) -> Result<Notification> {
        // Validate inputs
        if input.recipient_contact.is_empty() {
            return Err(NotificationError::ValidationError("Recipient contact is required".to_string()));
        }

        if input.content.is_empty() {
            return Err(NotificationError::ValidationError("Content is required".to_string()));
        }

        // If template is provided, populate content from template
        if let Some(template_id) = input.template_id {
            let template = self.template_repo.get_by_id(template_id).await?;

            // Apply template content if not provided
            if let Some(template_data_str) = &input.template_data {
                let template_data: HashMap<String, serde_json::Value> = serde_json::from_str(template_data_str)
                    .map_err(|e| NotificationError::ValidationError(format!("Invalid template data: {}", e)))?;

                // Simple variable replacement
                match input.notification_channel {
                    NotificationChannel::Email => {
                        if let Some(email_body) = &template.email_body {
                            input.content = self.replace_variables(email_body, &template_data);
                        }
                        if let Some(email_subject) = &template.email_subject {
                            input.subject = Some(self.replace_variables(email_subject, &template_data));
                        }
                    },
                    NotificationChannel::Sms => {
                        if let Some(sms_content) = &template.sms_content {
                            input.content = self.replace_variables(sms_content, &template_data);
                        }
                    },
                    NotificationChannel::Whatsapp => {
                        if let Some(whatsapp_content) = &template.whatsapp_content {
                            input.content = self.replace_variables(whatsapp_content, &template_data);
                        }
                    },
                    NotificationChannel::Push => {
                        if let Some(push_body) = &template.push_body {
                            input.content = self.replace_variables(push_body, &template_data);
                        }
                        if let Some(push_title) = &template.push_title {
                            input.subject = Some(self.replace_variables(push_title, &template_data));
                        }
                    },
                    _ => {}
                }
            }
        }

        // Check user preferences if recipient_id is provided
        if let Some(recipient_id) = input.recipient_id {
            match self.preference_repo.get_by_user(input.organization_id, recipient_id).await {
                Ok(pref) => {
                    // Check if channel is enabled
                    if !pref.is_channel_enabled(input.notification_channel) {
                        return Err(NotificationError::ChannelDisabled(
                            format!("Channel {:?} is disabled for this user", input.notification_channel)
                        ));
                    }

                    // Check quiet hours
                    if pref.is_in_quiet_hours() {
                        // Schedule for after quiet hours instead of blocking
                        // For now, just log and continue
                        tracing::info!("User in quiet hours, notification will be queued");
                    }
                },
                Err(_) => {
                    // No preferences found, use defaults (allow all)
                }
            }
        }

        // Create notification
        let notification = self.notification_repo.create(input, created_by).await?;

        // Log creation
        let _ = self.log_repo.log_event(notification.id, "CREATED", Some("Notification created".to_string())).await;

        // If no scheduled time, send immediately
        if notification.scheduled_at.is_none() {
            return self.process_notification(notification).await;
        }

        Ok(notification)
    }

    async fn process_notification(&self, mut notification: Notification) -> Result<Notification> {
        // Update status to queued
        notification = self.notification_repo.update_status(
            notification.id,
            NotificationStatus::Queued,
            None,
        ).await?;

        // Get provider configuration
        let provider_result = self.provider_repo.get_default(
            notification.organization_id,
            notification.notification_channel,
        ).await;

        match provider_result {
            Ok(provider) => {
                // Update status to sending
                notification = self.notification_repo.update_status(
                    notification.id,
                    NotificationStatus::Sending,
                    None,
                ).await?;

                // Send via provider (simplified - in production, use actual provider APIs)
                match self.send_via_provider(&notification, &provider).await {
                    Ok(_) => {
                        // Update status to sent
                        notification = self.notification_repo.update_status(
                            notification.id,
                            NotificationStatus::Sent,
                            Some("Successfully sent".to_string()),
                        ).await?;

                        let _ = self.log_repo.log_event(notification.id, "SENT", Some("Notification sent successfully".to_string())).await;
                    },
                    Err(e) => {
                        // Update status to failed
                        notification = self.notification_repo.update_status(
                            notification.id,
                            NotificationStatus::Failed,
                            Some(e.to_string()),
                        ).await?;

                        let _ = self.log_repo.log_event(notification.id, "FAILED", Some(e.to_string())).await;
                    }
                }
            },
            Err(_) => {
                // No provider configured
                notification = self.notification_repo.update_status(
                    notification.id,
                    NotificationStatus::Failed,
                    Some("No provider configured".to_string()),
                ).await?;

                let _ = self.log_repo.log_event(notification.id, "FAILED", Some("No provider configured".to_string())).await;
            }
        }

        Ok(notification)
    }

    async fn send_via_provider(&self, notification: &Notification, provider: &ProviderConfiguration) -> Result<()> {
        // In production, this would integrate with actual providers:
        // - Twilio for SMS
        // - SendGrid for Email
        // - WhatsApp Business API for WhatsApp
        // - FCM for Push notifications

        tracing::info!(
            "Sending {:?} notification via {} to {}",
            notification.notification_channel,
            provider.provider_name,
            notification.recipient_contact
        );

        // Simulate sending
        match notification.notification_channel {
            NotificationChannel::Email => {
                // Would use SendGrid API
                tracing::info!("Email: Subject: {:?}, Body: {}", notification.subject, &notification.content[..50.min(notification.content.len())]);
            },
            NotificationChannel::Sms => {
                // Would use Twilio API
                tracing::info!("SMS: {}", &notification.content[..50.min(notification.content.len())]);
            },
            NotificationChannel::Whatsapp => {
                // Would use WhatsApp Business API
                tracing::info!("WhatsApp: {}", &notification.content[..50.min(notification.content.len())]);
            },
            NotificationChannel::Push => {
                // Would use FCM
                tracing::info!("Push: {}", &notification.content[..50.min(notification.content.len())]);
            },
            NotificationChannel::InApp => {
                tracing::info!("In-App: {}", notification.content);
            }
        }

        Ok(())
    }

    fn replace_variables(&self, template: &str, data: &HashMap<String, serde_json::Value>) -> String {
        let mut result = template.to_string();
        for (key, value) in data {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
        result
    }

    pub async fn get_notification(&self, notification_id: Uuid) -> Result<Notification> {
        let notification = self.notification_repo.get_by_id(notification_id).await?;
        Ok(notification)
    }

    pub async fn list_notifications(&self, filter: Option<NotificationFilter>, page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Notification>> {
        let notifications = self.notification_repo.list(filter, page, page_size).await?;
        Ok(notifications)
    }

    pub async fn retry_notification(&self, notification_id: Uuid) -> Result<Notification> {
        let notification = self.notification_repo.get_by_id(notification_id).await?;

        if !notification.can_retry() {
            return Err(NotificationError::ValidationError("Cannot retry this notification".to_string()));
        }

        // Increment retry count
        let notification = self.notification_repo.increment_retry(notification_id).await?;

        // Process again
        self.process_notification(notification).await
    }

    // ============================================================================
    // Preference Operations
    // ============================================================================

    pub async fn update_preference(&self, input: UpdateNotificationPreferenceInput) -> Result<NotificationPreference> {
        let preference = self.preference_repo.upsert(input).await?;
        Ok(preference)
    }

    pub async fn get_preference(&self, organization_id: Uuid, user_id: Uuid) -> Result<NotificationPreference> {
        let preference = self.preference_repo.get_by_user(organization_id, user_id).await?;
        Ok(preference)
    }

    // ============================================================================
    // Log Operations
    // ============================================================================

    pub async fn get_notification_logs(&self, notification_id: Uuid) -> Result<Vec<NotificationLog>> {
        let logs = self.log_repo.get_by_notification(notification_id).await?;
        Ok(logs)
    }

    // ============================================================================
    // Provider Operations
    // ============================================================================

    pub async fn create_provider_config(&self, input: CreateProviderConfigInput, created_by: Uuid) -> Result<ProviderConfiguration> {
        let config = self.provider_repo.create(input, created_by).await?;
        Ok(config)
    }

    // ============================================================================
    // Batch Operations
    // ============================================================================

    pub async fn process_pending_notifications(&self, limit: i64) -> Result<Vec<Notification>> {
        let pending = self.notification_repo.get_pending(limit).await?;

        let mut processed = Vec::new();
        for notification in pending {
            match self.process_notification(notification).await {
                Ok(n) => processed.push(n),
                Err(e) => {
                    tracing::error!("Failed to process notification: {}", e);
                }
            }
        }

        Ok(processed)
    }
}
