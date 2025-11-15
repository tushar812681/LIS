use chrono::{Datelike, NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::*;
use crate::repository::AnalyticsRepository;
use common::error::Result;

#[derive(Clone)]
pub struct AnalyticsService {
    repository: AnalyticsRepository,
}

impl AnalyticsService {
    pub fn new(repository: AnalyticsRepository) -> Self {
        Self { repository }
    }

    // ========================================================================
    // Dashboard Generation
    // ========================================================================

    pub async fn generate_dashboard(&self, org_id: Uuid, role: &str) -> Result<Dashboard> {
        match role {
            "LAB_DIRECTOR" | "ADMIN" => self.generate_director_dashboard(org_id).await,
            "PATHOLOGIST" => self.generate_pathologist_dashboard(org_id).await,
            "LAB_TECHNICIAN" => self.generate_technician_dashboard(org_id).await,
            "FRONT_DESK" => self.generate_frontdesk_dashboard(org_id).await,
            _ => self.generate_basic_dashboard(org_id).await,
        }
    }

    async fn generate_director_dashboard(&self, org_id: Uuid) -> Result<Dashboard> {
        let today = Utc::now().naive_utc().date();

        // Get key metrics
        let today_samples = self.repository.get_daily_sample_count(org_id, today).await?;
        let pending_results = self.repository.get_pending_results_count(org_id).await?;
        let tat_compliance = self.repository.calculate_tat_compliance_rate(org_id, 30).await?;
        let today_revenue = self.repository.get_daily_revenue(org_id, today).await?;

        // Get trends
        let sample_trend_data = self.repository
            .get_sample_volume_trend(org_id, today - chrono::Duration::days(7), today)
            .await?;

        let revenue_trend_data = self.repository
            .get_revenue_trend(org_id, today - chrono::Duration::days(30), today)
            .await?;

        Ok(Dashboard {
            role: "LAB_DIRECTOR".to_string(),
            metrics: vec![
                Metric {
                    name: "Today's Samples".to_string(),
                    value: today_samples.to_string(),
                    unit: Some("samples".to_string()),
                    trend: None,
                    status: MetricStatus::Normal,
                },
                Metric {
                    name: "Pending Results".to_string(),
                    value: pending_results.to_string(),
                    unit: Some("results".to_string()),
                    trend: None,
                    status: if pending_results > 100 {
                        MetricStatus::Warning
                    } else {
                        MetricStatus::Normal
                    },
                },
                Metric {
                    name: "TAT Compliance".to_string(),
                    value: format!("{:.1}", tat_compliance),
                    unit: Some("%".to_string()),
                    trend: None,
                    status: if tat_compliance >= 95.0 {
                        MetricStatus::OnTarget
                    } else if tat_compliance >= 85.0 {
                        MetricStatus::Warning
                    } else {
                        MetricStatus::Critical
                    },
                },
                Metric {
                    name: "Today's Revenue".to_string(),
                    value: format!("₹{}", today_revenue),
                    unit: None,
                    trend: None,
                    status: MetricStatus::Normal,
                },
            ],
            charts: vec![
                Chart {
                    chart_type: ChartType::Line,
                    title: "Sample Volume (7 Days)".to_string(),
                    data: serde_json::to_value(&sample_trend_data).unwrap_or_default(),
                },
                Chart {
                    chart_type: ChartType::Bar,
                    title: "Revenue Trend (30 Days)".to_string(),
                    data: serde_json::to_value(&revenue_trend_data).unwrap_or_default(),
                },
            ],
            alerts: vec![],
        })
    }

    async fn generate_pathologist_dashboard(&self, org_id: Uuid) -> Result<Dashboard> {
        let pending_verifications = self.repository.get_pending_verifications_count(org_id).await?;
        let critical_count = self.repository.get_critical_result_count(org_id, 1).await?;

        Ok(Dashboard {
            role: "PATHOLOGIST".to_string(),
            metrics: vec![
                Metric {
                    name: "Pending Verifications".to_string(),
                    value: pending_verifications.to_string(),
                    unit: Some("results".to_string()),
                    trend: None,
                    status: if pending_verifications > 50 {
                        MetricStatus::Warning
                    } else {
                        MetricStatus::Normal
                    },
                },
                Metric {
                    name: "Critical Values Today".to_string(),
                    value: critical_count.to_string(),
                    unit: None,
                    trend: None,
                    status: if critical_count > 0 {
                        MetricStatus::Critical
                    } else {
                        MetricStatus::Normal
                    },
                },
            ],
            charts: vec![],
            alerts: if critical_count > 0 {
                vec![Alert {
                    level: AlertLevel::Critical,
                    title: "Critical Values Detected".to_string(),
                    message: format!("{} critical values require immediate attention", critical_count),
                    timestamp: Utc::now().to_rfc3339(),
                }]
            } else {
                vec![]
            },
        })
    }

    async fn generate_technician_dashboard(&self, org_id: Uuid) -> Result<Dashboard> {
        let today = Utc::now().naive_utc().date();
        let today_samples = self.repository.get_daily_sample_count(org_id, today).await?;
        let pending_results = self.repository.get_pending_results_count(org_id).await?;

        Ok(Dashboard {
            role: "LAB_TECHNICIAN".to_string(),
            metrics: vec![
                Metric {
                    name: "Samples Today".to_string(),
                    value: today_samples.to_string(),
                    unit: None,
                    trend: None,
                    status: MetricStatus::Normal,
                },
                Metric {
                    name: "Pending Results".to_string(),
                    value: pending_results.to_string(),
                    unit: None,
                    trend: None,
                    status: MetricStatus::Normal,
                },
            ],
            charts: vec![],
            alerts: vec![],
        })
    }

    async fn generate_frontdesk_dashboard(&self, org_id: Uuid) -> Result<Dashboard> {
        let today = Utc::now().naive_utc().date();
        let today_samples = self.repository.get_daily_sample_count(org_id, today).await?;

        Ok(Dashboard {
            role: "FRONT_DESK".to_string(),
            metrics: vec![Metric {
                name: "Samples Registered Today".to_string(),
                value: today_samples.to_string(),
                unit: None,
                trend: None,
                status: MetricStatus::Normal,
            }],
            charts: vec![],
            alerts: vec![],
        })
    }

    async fn generate_basic_dashboard(&self, org_id: Uuid) -> Result<Dashboard> {
        let today = Utc::now().naive_utc().date();
        let today_samples = self.repository.get_daily_sample_count(org_id, today).await?;

        Ok(Dashboard {
            role: "BASIC".to_string(),
            metrics: vec![Metric {
                name: "Samples Today".to_string(),
                value: today_samples.to_string(),
                unit: None,
                trend: None,
                status: MetricStatus::Normal,
            }],
            charts: vec![],
            alerts: vec![],
        })
    }

    // ========================================================================
    // TAT Analytics
    // ========================================================================

    pub async fn get_tat_analytics(&self, org_id: Uuid, days: i32) -> Result<TATAnalytics> {
        let compliance_rate = self.repository.calculate_tat_compliance_rate(org_id, days).await?;
        let avg_tat = self.repository.get_average_tat_hours(org_id, days).await?;

        let end_date = Utc::now();
        let start_date = end_date - chrono::Duration::days(days as i64);

        Ok(TATAnalytics {
            period_start: start_date,
            period_end: end_date,
            total_orders: 0, // TODO: Get from repository
            mean_tat_hours: avg_tat,
            median_tat_hours: avg_tat, // Simplified
            p95_tat_hours: avg_tat * 1.5, // Approximation
            compliance_rate,
            by_priority: vec![],
            by_department: vec![],
        })
    }

    // ========================================================================
    // Revenue Analytics
    // ========================================================================

    pub async fn get_revenue_analytics(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<RevenueAnalytics> {
        let revenue_data = self.repository
            .get_revenue_trend(org_id, start_date, end_date)
            .await?;

        let total_revenue: rust_decimal::Decimal = revenue_data.iter().map(|d| d.revenue).sum();
        let total_orders: i64 = revenue_data.iter().map(|d| d.order_count).sum();

        let avg_order_value = if total_orders > 0 {
            total_revenue / rust_decimal::Decimal::from(total_orders)
        } else {
            rust_decimal::Decimal::ZERO
        };

        Ok(RevenueAnalytics {
            period: format!("{} to {}", start_date, end_date),
            total_revenue,
            total_orders,
            average_order_value: avg_order_value,
            revenue_by_day: revenue_data,
            top_tests: vec![],
            payment_method_breakdown: vec![],
        })
    }

    // ========================================================================
    // Sample Volume Analytics
    // ========================================================================

    pub async fn get_sample_volume_analytics(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<SampleVolumeAnalytics> {
        let trend_data = self.repository
            .get_sample_volume_trend(org_id, start_date, end_date)
            .await?;

        let total_samples: i64 = trend_data.iter().map(|d| d.count).sum();
        let days = (end_date - start_date).num_days() + 1;
        let daily_average = total_samples as f64 / days as f64;

        let peak_day = trend_data
            .iter()
            .max_by_key(|d| d.count)
            .map(|d| (d.date, d.count))
            .unwrap_or((start_date, 0));

        Ok(SampleVolumeAnalytics {
            period: format!("{} to {}", start_date, end_date),
            total_samples,
            daily_average,
            peak_day: peak_day.0,
            peak_day_count: peak_day.1,
            by_type: vec![],
            by_status: vec![],
            trend_data,
        })
    }

    // ========================================================================
    // Quality Metrics
    // ========================================================================

    pub async fn get_quality_metrics(&self, org_id: Uuid, days: i32) -> Result<QualityMetrics> {
        let rejection_rate = self.repository.get_sample_rejection_rate(org_id, days).await?;
        let critical_count = self.repository.get_critical_result_count(org_id, days).await?;
        let tat_compliance = self.repository.calculate_tat_compliance_rate(org_id, days).await?;

        Ok(QualityMetrics {
            period: format!("Last {} days", days),
            sample_rejection_rate: rejection_rate,
            result_amendment_rate: 0.0, // TODO: Implement
            critical_value_count: critical_count,
            qc_violation_count: 0, // TODO: Implement
            tat_compliance_rate: tat_compliance,
        })
    }

    // ========================================================================
    // Equipment Utilization
    // ========================================================================

    pub async fn get_equipment_utilization(&self, org_id: Uuid, days: i32) -> Result<Vec<EquipmentUtilization>> {
        self.repository.get_equipment_utilization(org_id, days).await
    }

    // ========================================================================
    // Saved Reports
    // ========================================================================

    pub async fn create_saved_report(
        &self,
        input: CreateSavedReportInput,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<SavedReport> {
        input.validate()?;
        self.repository.create_saved_report(input, org_id, user_id).await
    }

    pub async fn get_saved_reports(&self, org_id: Uuid) -> Result<Vec<SavedReport>> {
        self.repository.get_saved_reports(org_id).await
    }

    // ========================================================================
    // Operational Metrics
    // ========================================================================

    pub async fn get_operational_metrics(&self, org_id: Uuid, days: i32) -> Result<OperationalMetrics> {
        let today = Utc::now().naive_utc().date();
        let start_date = today - chrono::Duration::days(days as i64);

        let total_samples = self.repository
            .get_sample_volume_trend(org_id, start_date, today)
            .await?
            .iter()
            .map(|d| d.count)
            .sum();

        let pending_results = self.repository.get_pending_results_count(org_id).await?;
        let pending_verifications = self.repository.get_pending_verifications_count(org_id).await?;
        let critical_alerts = self.repository.get_critical_result_count(org_id, 1).await?;

        Ok(OperationalMetrics {
            period: format!("Last {} days", days),
            total_orders: 0, // TODO: Implement
            total_samples,
            total_results: 0, // TODO: Implement
            pending_results,
            pending_verifications,
            critical_alerts,
        })
    }
}
