use async_graphql::{SimpleObject, Enum, InputObject};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================================
// Dashboard Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Dashboard {
    pub role: String,
    pub metrics: Vec<Metric>,
    pub charts: Vec<Chart>,
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Metric {
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
    pub trend: Option<Trend>,
    pub status: MetricStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, PartialEq, Eq)]
pub enum MetricStatus {
    OnTarget,
    Warning,
    Critical,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Trend {
    pub direction: TrendDirection,
    pub percentage: f64,
    pub period: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, PartialEq, Eq)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Chart {
    pub chart_type: ChartType,
    pub title: String,
    #[graphql(skip)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, PartialEq, Eq)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Doughnut,
    Area,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Alert {
    pub level: AlertLevel,
    pub title: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

// ============================================================================
// KPI Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KPIDefinition {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub kpi_name: String,
    pub kpi_code: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub calculation_method: String,
    pub sql_query: Option<String>,
    pub formula: Option<String>,
    pub unit: Option<String>,
    pub target_value: Option<rust_decimal::Decimal>,
    pub warning_threshold: Option<rust_decimal::Decimal>,
    pub critical_threshold: Option<rust_decimal::Decimal>,
    pub is_higher_better: bool,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KPIValue {
    pub id: Uuid,
    pub kpi_id: Uuid,
    pub organization_id: Uuid,
    pub value_date: NaiveDate,
    pub calculated_value: rust_decimal::Decimal,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub calculated_at: DateTime<Utc>,
}

// ============================================================================
// TAT Analytics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct TATAnalytics {
    #[graphql(skip)]
    pub period_start: DateTime<Utc>,
    #[graphql(skip)]
    pub period_end: DateTime<Utc>,
    pub total_orders: i64,
    pub mean_tat_hours: f64,
    pub median_tat_hours: f64,
    pub p95_tat_hours: f64,
    pub compliance_rate: f64,
    pub by_priority: Vec<PriorityTAT>,
    pub by_department: Vec<DepartmentTAT>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PriorityTAT {
    pub priority: String,
    pub count: i64,
    pub mean_tat_hours: f64,
    pub compliance_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DepartmentTAT {
    pub department: String,
    pub count: i64,
    pub mean_tat_hours: f64,
}

// ============================================================================
// Revenue Analytics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct RevenueAnalytics {
    pub period: String,
    pub total_revenue: rust_decimal::Decimal,
    pub total_orders: i64,
    pub average_order_value: rust_decimal::Decimal,
    pub revenue_by_day: Vec<DailyRevenue>,
    pub top_tests: Vec<TestRevenue>,
    pub payment_method_breakdown: Vec<PaymentMethodRevenue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DailyRevenue {
    pub date: NaiveDate,
    pub revenue: rust_decimal::Decimal,
    pub order_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct TestRevenue {
    pub test_name: String,
    pub test_count: i64,
    pub total_revenue: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PaymentMethodRevenue {
    pub payment_method: String,
    pub revenue: rust_decimal::Decimal,
    pub percentage: f64,
}

// ============================================================================
// Sample Volume Analytics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SampleVolumeAnalytics {
    pub period: String,
    pub total_samples: i64,
    pub daily_average: f64,
    pub peak_day: NaiveDate,
    pub peak_day_count: i64,
    pub by_type: Vec<SampleTypeVolume>,
    pub by_status: Vec<SampleStatusVolume>,
    pub trend_data: Vec<DailySampleVolume>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SampleTypeVolume {
    pub sample_type: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SampleStatusVolume {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DailySampleVolume {
    pub date: NaiveDate,
    pub count: i64,
}

// ============================================================================
// Equipment Utilization
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct EquipmentUtilization {
    pub equipment_id: Uuid,
    pub equipment_name: String,
    pub total_capacity: i32,
    pub tests_processed: i32,
    pub utilization_percentage: f64,
    pub average_turnaround_minutes: f64,
    pub downtime_hours: f64,
}

// ============================================================================
// Custom Reports
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct SavedReport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub created_by: Uuid,
    pub report_name: String,
    pub report_type: String,
    pub description: Option<String>,
    pub date_range_type: Option<String>,
    pub custom_date_from: Option<NaiveDate>,
    pub custom_date_to: Option<NaiveDate>,
    #[graphql(skip)]
    pub filters: Option<serde_json::Value>,
    pub group_by: Option<String>,
    pub aggregation: Option<String>,
    pub is_scheduled: bool,
    pub schedule_frequency: Option<String>,
    #[graphql(skip)]
    pub schedule_time: Option<chrono::NaiveTime>,
    #[graphql(skip)]
    pub recipients: Option<serde_json::Value>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
    #[graphql(skip)]
    pub last_generated_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportExecutionResult {
    pub report_id: Uuid,
    pub rows: Vec<serde_json::Value>,
    pub row_count: i64,
    pub execution_time_ms: u128,
    pub generated_at: DateTime<Utc>,
}

// ============================================================================
// Input Types for GraphQL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeInput {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl DateRangeInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        if self.end_date < self.start_date {
            return Err(common::error::Error::InvalidInput(
                "End date must be after start date".to_string()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct CreateSavedReportInput {
    pub report_name: String,
    pub report_type: String,
    pub description: Option<String>,
    pub date_range_type: String,
    pub custom_date_from: Option<String>,
    pub custom_date_to: Option<String>,
    pub filters: Option<String>,
    pub group_by: Option<String>,
}

impl CreateSavedReportInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        if self.report_name.trim().is_empty() {
            return Err(common::error::Error::InvalidInput(
                "Report name is required".to_string()
            ));
        }

        if self.date_range_type == "CUSTOM" {
            if self.custom_date_from.is_none() || self.custom_date_to.is_none() {
                return Err(common::error::Error::InvalidInput(
                    "Custom date range requires both from and to dates".to_string()
                ));
            }

            // Validate date format if provided
            if let Some(ref from_str) = self.custom_date_from {
                NaiveDate::parse_from_str(from_str, "%Y-%m-%d")
                    .map_err(|_| common::error::Error::InvalidInput(
                        "Invalid custom_date_from format. Use YYYY-MM-DD".to_string()
                    ))?;
            }
            if let Some(ref to_str) = self.custom_date_to {
                NaiveDate::parse_from_str(to_str, "%Y-%m-%d")
                    .map_err(|_| common::error::Error::InvalidInput(
                        "Invalid custom_date_to format. Use YYYY-MM-DD".to_string()
                    ))?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// Quality Metrics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct QualityMetrics {
    pub period: String,
    pub sample_rejection_rate: f64,
    pub result_amendment_rate: f64,
    pub critical_value_count: i64,
    pub qc_violation_count: i64,
    pub tat_compliance_rate: f64,
}

// ============================================================================
// Operational Metrics
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct OperationalMetrics {
    pub period: String,
    pub total_orders: i64,
    pub total_samples: i64,
    pub total_results: i64,
    pub pending_results: i64,
    pub pending_verifications: i64,
    pub critical_alerts: i64,
}
