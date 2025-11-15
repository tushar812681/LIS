use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;
use common::error::{Error, Result};

#[derive(Clone)]
pub struct AnalyticsRepository {
    pool: PgPool,
}

impl AnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Sample Volume Analytics
    // ========================================================================

    pub async fn get_daily_sample_count(&self, org_id: Uuid, date: NaiveDate) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sample WHERE organization_id = $1 AND DATE(created_at) = $2"
        )
        .bind(org_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    pub async fn get_sample_volume_trend(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailySampleVolume>> {
        let results = sqlx::query_as::<_, (NaiveDate, i64)>(
            r#"
            SELECT
                DATE(created_at) as date,
                COUNT(*) as count
            FROM sample
            WHERE organization_id = $1
              AND DATE(created_at) BETWEEN $2 AND $3
              AND is_deleted = FALSE
            GROUP BY DATE(created_at)
            ORDER BY DATE(created_at)
            "#
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(date, count)| DailySampleVolume { date, count })
            .collect())
    }

    // ========================================================================
    // TAT Analytics
    // ========================================================================

    pub async fn calculate_tat_compliance_rate(&self, org_id: Uuid, days: i32) -> Result<f64> {
        let result: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(CASE WHEN actual_completion_at <= estimated_completion_at THEN 1 END) as on_time
            FROM lab_order
            WHERE organization_id = $1
              AND order_status = 'COMPLETED'
              AND created_at >= NOW() - ($2 || ' days')::INTERVAL
            "#
        )
        .bind(org_id)
        .bind(days)
        .fetch_one(&self.pool)
        .await?;

        if result.0 == 0 {
            return Ok(0.0);
        }

        Ok((result.1 as f64 / result.0 as f64) * 100.0)
    }

    pub async fn get_average_tat_hours(&self, org_id: Uuid, days: i32) -> Result<f64> {
        let result: (Option<f64>,) = sqlx::query_as(
            r#"
            SELECT
                AVG(EXTRACT(EPOCH FROM (actual_completion_at - created_at)) / 3600) as avg_hours
            FROM lab_order
            WHERE organization_id = $1
              AND order_status = 'COMPLETED'
              AND actual_completion_at IS NOT NULL
              AND created_at >= NOW() - ($2 || ' days')::INTERVAL
            "#
        )
        .bind(org_id)
        .bind(days)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0.unwrap_or(0.0))
    }

    // ========================================================================
    // Revenue Analytics
    // ========================================================================

    pub async fn get_daily_revenue(&self, org_id: Uuid, date: NaiveDate) -> Result<rust_decimal::Decimal> {
        let result: (Option<rust_decimal::Decimal>,) = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(final_amount), 0) as revenue
            FROM invoice
            WHERE organization_id = $1
              AND DATE(invoice_date) = $2
              AND invoice_status IN ('PAID', 'PARTIALLY_PAID')
            "#
        )
        .bind(org_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0.unwrap_or_else(|| rust_decimal::Decimal::ZERO))
    }

    pub async fn get_revenue_trend(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyRevenue>> {
        let results = sqlx::query_as::<_, (NaiveDate, rust_decimal::Decimal, i64)>(
            r#"
            SELECT
                DATE(invoice_date) as date,
                COALESCE(SUM(final_amount), 0) as revenue,
                COUNT(*) as order_count
            FROM invoice
            WHERE organization_id = $1
              AND DATE(invoice_date) BETWEEN $2 AND $3
              AND invoice_status IN ('PAID', 'PARTIALLY_PAID')
            GROUP BY DATE(invoice_date)
            ORDER BY DATE(invoice_date)
            "#
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(date, revenue, order_count)| DailyRevenue {
                date,
                revenue,
                order_count,
            })
            .collect())
    }

    // ========================================================================
    // KPI Operations
    // ========================================================================

    pub async fn get_kpi_definitions(&self, org_id: Uuid) -> Result<Vec<KPIDefinition>> {
        let kpis = sqlx::query_as::<_, KPIDefinition>(
            r#"
            SELECT *
            FROM kpi_definition
            WHERE organization_id = $1 AND is_active = TRUE
            ORDER BY kpi_name
            "#
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(kpis)
    }

    pub async fn get_kpi_value(
        &self,
        kpi_id: Uuid,
        value_date: NaiveDate,
    ) -> Result<Option<KPIValue>> {
        let value = sqlx::query_as::<_, KPIValue>(
            r#"
            SELECT *
            FROM kpi_value
            WHERE kpi_id = $1 AND value_date = $2
            "#
        )
        .bind(kpi_id)
        .bind(value_date)
        .fetch_optional(&self.pool)
        .await?;

        Ok(value)
    }

    pub async fn save_kpi_value(&self, value: &KPIValue) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO kpi_value (id, kpi_id, organization_id, value_date, calculated_value, status, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (kpi_id, value_date)
            DO UPDATE SET
                calculated_value = EXCLUDED.calculated_value,
                status = EXCLUDED.status,
                metadata = EXCLUDED.metadata,
                calculated_at = NOW()
            "#
        )
        .bind(value.id)
        .bind(value.kpi_id)
        .bind(value.organization_id)
        .bind(value.value_date)
        .bind(value.calculated_value)
        .bind(&value.status)
        .bind(&value.metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Equipment Utilization
    // ========================================================================

    pub async fn get_equipment_utilization(
        &self,
        org_id: Uuid,
        days: i32,
    ) -> Result<Vec<EquipmentUtilization>> {
        let results = sqlx::query_as::<_, (Uuid, String, i32, i64)>(
            r#"
            SELECT
                e.id,
                e.equipment_name,
                e.max_capacity_per_day,
                COUNT(r.id) as tests_processed
            FROM equipment e
            LEFT JOIN test_result r ON r.equipment_id = e.id
                AND r.created_at >= NOW() - ($2 || ' days')::INTERVAL
            WHERE e.organization_id = $1
              AND e.is_active = TRUE
            GROUP BY e.id, e.equipment_name, e.max_capacity_per_day
            "#
        )
        .bind(org_id)
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(id, name, capacity, processed)| {
                let utilization = if capacity > 0 {
                    (processed as f64 / capacity as f64) * 100.0
                } else {
                    0.0
                };

                EquipmentUtilization {
                    equipment_id: id,
                    equipment_name: name,
                    total_capacity: capacity,
                    tests_processed: processed as i32,
                    utilization_percentage: utilization,
                    average_turnaround_minutes: 0.0, // TODO: Calculate from result timestamps
                    downtime_hours: 0.0, // TODO: Calculate from equipment logs
                }
            })
            .collect())
    }

    // ========================================================================
    // Quality Metrics
    // ========================================================================

    pub async fn get_sample_rejection_rate(&self, org_id: Uuid, days: i32) -> Result<f64> {
        let result: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(CASE WHEN sample_status = 'REJECTED' THEN 1 END) as rejected
            FROM sample
            WHERE organization_id = $1
              AND created_at >= NOW() - ($2 || ' days')::INTERVAL
            "#
        )
        .bind(org_id)
        .bind(days)
        .fetch_one(&self.pool)
        .await?;

        if result.0 == 0 {
            return Ok(0.0);
        }

        Ok((result.1 as f64 / result.0 as f64) * 100.0)
    }

    pub async fn get_critical_result_count(&self, org_id: Uuid, days: i32) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM test_result
            WHERE organization_id = $1
              AND is_critical = TRUE
              AND created_at >= NOW() - ($2 || ' days')::INTERVAL
            "#
        )
        .bind(org_id)
        .bind(days)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // Saved Reports
    // ========================================================================

    pub async fn create_saved_report(&self, input: CreateSavedReportInput, org_id: Uuid, user_id: Uuid) -> Result<SavedReport> {
        let id = Uuid::new_v4();

        let report = sqlx::query_as::<_, SavedReport>(
            r#"
            INSERT INTO saved_report (
                id, organization_id, created_by, report_name, report_type,
                description, date_range_type, custom_date_from, custom_date_to,
                filters, group_by, is_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, TRUE)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(org_id)
        .bind(user_id)
        .bind(&input.report_name)
        .bind(&input.report_type)
        .bind(&input.description)
        .bind(&input.date_range_type)
        .bind(input.custom_date_from)
        .bind(input.custom_date_to)
        .bind(&input.filters)
        .bind(&input.group_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(report)
    }

    pub async fn get_saved_reports(&self, org_id: Uuid) -> Result<Vec<SavedReport>> {
        let reports = sqlx::query_as::<_, SavedReport>(
            "SELECT * FROM saved_report WHERE organization_id = $1 AND is_active = TRUE ORDER BY created_at DESC"
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(reports)
    }

    // ========================================================================
    // Pending/Active Counts
    // ========================================================================

    pub async fn get_pending_results_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM test_result WHERE organization_id = $1 AND result_status = 'PENDING'"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    pub async fn get_pending_verifications_count(&self, org_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM test_result WHERE organization_id = $1 AND verification_status = 'PENDING_REVIEW'"
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }
}
