use async_graphql::{Context, Object, Result as GqlResult, ID, ErrorExtensions};
use crate::domain::*;
use crate::service::{NotificationService, NotificationError};
use uuid::Uuid;
use std::str::FromStr;

impl ErrorExtensions for NotificationError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ============================================================================
    // Template Queries
    // ============================================================================

    async fn notification_template(&self, ctx: &Context<'_>, id: ID) -> GqlResult<NotificationTemplate> {
        let service = ctx.data::<NotificationService>()?;
        let template_id = Uuid::from_str(&id)?;
        let template = service.get_template(template_id).await?;
        Ok(template)
    }

    async fn notification_template_by_code(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        template_code: String,
    ) -> GqlResult<NotificationTemplate> {
        let service = ctx.data::<NotificationService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let template = service.get_template_by_code(org_id, &template_code).await?;
        Ok(template)
    }

    async fn notification_templates(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        template_type: Option<TemplateType>,
        is_active: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<NotificationTemplate>> {
        let service = ctx.data::<NotificationService>()?;

        let filter = if organization_id.is_some() || template_type.is_some() || is_active.is_some() {
            Some(NotificationTemplateFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                template_type,
                is_active,
            })
        } else {
            None
        };

        let templates = service.list_templates(filter, page, page_size).await?;
        Ok(templates)
    }

    // ============================================================================
    // Notification Queries
    // ============================================================================

    async fn notification(&self, ctx: &Context<'_>, id: ID) -> GqlResult<Notification> {
        let service = ctx.data::<NotificationService>()?;
        let notification_id = Uuid::from_str(&id)?;
        let notification = service.get_notification(notification_id).await?;
        Ok(notification)
    }

    async fn notifications(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<ID>,
        recipient_id: Option<ID>,
        notification_channel: Option<NotificationChannel>,
        notification_status: Option<NotificationStatus>,
        from_date: Option<String>,
        to_date: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GqlResult<Vec<Notification>> {
        let service = ctx.data::<NotificationService>()?;

        let filter = if organization_id.is_some() || recipient_id.is_some() || notification_channel.is_some()
            || notification_status.is_some() || from_date.is_some() || to_date.is_some() {
            Some(NotificationFilter {
                organization_id: organization_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                recipient_id: recipient_id.as_ref().and_then(|id| Uuid::from_str(id).ok()),
                notification_channel,
                notification_status,
                from_date: from_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
                to_date: to_date.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
            })
        } else {
            None
        };

        let notifications = service.list_notifications(filter, page, page_size).await?;
        Ok(notifications)
    }

    // ============================================================================
    // Preference Queries
    // ============================================================================

    async fn notification_preference(
        &self,
        ctx: &Context<'_>,
        organization_id: ID,
        user_id: ID,
    ) -> GqlResult<NotificationPreference> {
        let service = ctx.data::<NotificationService>()?;
        let org_id = Uuid::from_str(&organization_id)?;
        let user_uuid = Uuid::from_str(&user_id)?;
        let preference = service.get_preference(org_id, user_uuid).await?;
        Ok(preference)
    }

    // ============================================================================
    // Log Queries
    // ============================================================================

    async fn notification_logs(&self, ctx: &Context<'_>, notification_id: ID) -> GqlResult<Vec<NotificationLog>> {
        let service = ctx.data::<NotificationService>()?;
        let notif_id = Uuid::from_str(&notification_id)?;
        let logs = service.get_notification_logs(notif_id).await?;
        Ok(logs)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ============================================================================
    // Template Mutations
    // ============================================================================

    async fn create_notification_template(
        &self,
        ctx: &Context<'_>,
        input: CreateNotificationTemplateInput,
        created_by: ID,
    ) -> GqlResult<NotificationTemplate> {
        let service = ctx.data::<NotificationService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let template = service.create_template(input, creator_id).await?;
        Ok(template)
    }

    // ============================================================================
    // Notification Mutations
    // ============================================================================

    async fn send_notification(
        &self,
        ctx: &Context<'_>,
        input: SendNotificationInput,
        created_by: Option<ID>,
    ) -> GqlResult<Notification> {
        let service = ctx.data::<NotificationService>()?;
        let creator_id = created_by.as_ref().and_then(|id| Uuid::from_str(id).ok());
        let notification = service.send_notification(input, creator_id).await?;
        Ok(notification)
    }

    async fn retry_notification(
        &self,
        ctx: &Context<'_>,
        notification_id: ID,
    ) -> GqlResult<Notification> {
        let service = ctx.data::<NotificationService>()?;
        let notif_id = Uuid::from_str(&notification_id)?;
        let notification = service.retry_notification(notif_id).await?;
        Ok(notification)
    }

    // ============================================================================
    // Preference Mutations
    // ============================================================================

    async fn update_notification_preference(
        &self,
        ctx: &Context<'_>,
        input: UpdateNotificationPreferenceInput,
    ) -> GqlResult<NotificationPreference> {
        let service = ctx.data::<NotificationService>()?;
        let preference = service.update_preference(input).await?;
        Ok(preference)
    }

    // ============================================================================
    // Provider Mutations
    // ============================================================================

    async fn create_provider_config(
        &self,
        ctx: &Context<'_>,
        input: CreateProviderConfigInput,
        created_by: ID,
    ) -> GqlResult<ProviderConfiguration> {
        let service = ctx.data::<NotificationService>()?;
        let creator_id = Uuid::from_str(&created_by)?;
        let config = service.create_provider_config(input, creator_id).await?;
        Ok(config)
    }

    // ============================================================================
    // Batch Operations
    // ============================================================================

    async fn process_pending_notifications(
        &self,
        ctx: &Context<'_>,
        limit: i64,
    ) -> GqlResult<Vec<Notification>> {
        let service = ctx.data::<NotificationService>()?;
        let notifications = service.process_pending_notifications(limit).await?;
        Ok(notifications)
    }
}
