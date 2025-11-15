use async_graphql::{Context, Object, Result};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;
use crate::repository::AnalyticsRepository;
use crate::service::AnalyticsService;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get role-based dashboard for a user
    async fn dashboard(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        role: String,
    ) -> Result<Dashboard> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let dashboard = service.generate_dashboard(org_id, &role).await?;
        Ok(dashboard)
    }

    /// Get TAT analytics for a period
    async fn tat_analytics(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        days: i32,
    ) -> Result<TATAnalytics> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let analytics = service.get_tat_analytics(org_id, days).await?;
        Ok(analytics)
    }

    /// Get revenue analytics for a date range
    async fn revenue_analytics(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        start_date: String,
        end_date: String,
    ) -> Result<RevenueAnalytics> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")?;
        let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")?;

        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let analytics = service.get_revenue_analytics(org_id, start, end).await?;
        Ok(analytics)
    }

    /// Get sample volume analytics
    async fn sample_volume_analytics(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        start_date: String,
        end_date: String,
    ) -> Result<SampleVolumeAnalytics> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")?;
        let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")?;

        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let analytics = service.get_sample_volume_analytics(org_id, start, end).await?;
        Ok(analytics)
    }

    /// Get quality metrics
    async fn quality_metrics(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        days: i32,
    ) -> Result<QualityMetrics> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let metrics = service.get_quality_metrics(org_id, days).await?;
        Ok(metrics)
    }

    /// Get equipment utilization metrics
    async fn equipment_utilization(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        days: Option<i32>,
    ) -> Result<Vec<EquipmentUtilization>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let days = days.unwrap_or(30);

        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let utilization = service.get_equipment_utilization(org_id, days).await?;
        Ok(utilization)
    }

    /// Get operational metrics
    async fn operational_metrics(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        days: Option<i32>,
    ) -> Result<OperationalMetrics> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let days = days.unwrap_or(7);

        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let metrics = service.get_operational_metrics(org_id, days).await?;
        Ok(metrics)
    }

    /// Get saved reports for an organization
    async fn saved_reports(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
    ) -> Result<Vec<SavedReport>> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let reports = service.get_saved_reports(org_id).await?;
        Ok(reports)
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new saved report
    async fn create_saved_report(
        &self,
        ctx: &Context<'_>,
        input: CreateSavedReportInput,
        organization_id: String,
        created_by: String,
    ) -> Result<SavedReport> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let user_id = Uuid::parse_str(&created_by)?;

        let pool = ctx.data::<PgPool>()?;
        let repository = AnalyticsRepository::new(pool.clone());
        let service = AnalyticsService::new(repository);

        let report = service.create_saved_report(input, org_id, user_id).await?;

        tracing::info!("Saved report created: {} ({})", report.report_name, report.id);
        Ok(report)
    }
}
