use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use common::error::{Error, Result};
use common::types::{SampleStatus};

use crate::domain::*;

// ============================================================================
// Sample Repository
// ============================================================================

#[derive(Clone)]
pub struct SampleRepository {
    pool: PgPool,
}

impl SampleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new sample
    pub async fn create(&self, input: CreateSampleInput, org_id: Uuid, user_id: Uuid) -> Result<Sample> {
        input.validate()?;

        // Generate sample ID with Luhn checksum
        let sample_id = self.generate_sample_id(&org_id, &input.sample_type).await?;

        let sample = sqlx::query_as::<_, Sample>(
            r#"
            INSERT INTO sample (
                id, sample_id, patient_id, order_id, organization_id,
                specimen_type, sample_status, priority,
                collection_date_time, collection_site, collection_method, collection_notes,
                volume_ml, requires_fasting, fasting_hours, special_instructions,
                created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&sample_id)
        .bind(input.patient_id)
        .bind(input.order_id)
        .bind(org_id)
        .bind(&input.sample_type)
        .bind(SampleStatus::Pending)
        .bind(&input.priority)
        .bind(input.collection_date_time)
        .bind(&input.collection_site)
        .bind(&input.collection_method)
        .bind(&input.collection_notes)
        .bind(input.volume_ml)
        .bind(input.requires_fasting)
        .bind(input.fasting_hours)
        .bind(&input.special_instructions)
        .bind(user_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Find sample by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Sample>> {
        let sample = sqlx::query_as::<_, Sample>(
            "SELECT * FROM sample WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Find sample by sample ID
    pub async fn find_by_sample_id(&self, sample_id: &str) -> Result<Option<Sample>> {
        let sample = sqlx::query_as::<_, Sample>(
            "SELECT * FROM sample WHERE sample_id = $1 AND is_deleted = FALSE"
        )
        .bind(sample_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Find sample by barcode
    pub async fn find_by_barcode(&self, barcode: &str) -> Result<Option<Sample>> {
        let sample = sqlx::query_as::<_, Sample>(
            "SELECT * FROM sample WHERE barcode = $1 AND is_deleted = FALSE"
        )
        .bind(barcode)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Find samples by patient
    pub async fn find_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<Sample>> {
        let samples = sqlx::query_as::<_, Sample>(
            r#"
            SELECT * FROM sample
            WHERE patient_id = $1 AND is_deleted = FALSE
            ORDER BY collection_date_time DESC
            LIMIT $2
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(samples)
    }

    /// Find samples by order
    pub async fn find_by_order(&self, order_id: Uuid) -> Result<Vec<Sample>> {
        let samples = sqlx::query_as::<_, Sample>(
            r#"
            SELECT * FROM sample
            WHERE order_id = $1 AND is_deleted = FALSE
            ORDER BY created_at DESC
            "#
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(samples)
    }

    /// Search samples with filters
    pub async fn search(&self, filter: SampleFilter, org_id: Uuid, limit: i64) -> Result<Vec<Sample>> {
        let mut query = String::from(
            "SELECT * FROM sample WHERE organization_id = $1 AND is_deleted = FALSE"
        );
        let mut param_count = 1;

        // Build dynamic query based on filters
        if filter.patient_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND patient_id = ${}", param_count));
        }
        if filter.sample_status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND sample_status = ${}", param_count));
        }
        if filter.is_rejected.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND is_rejected = ${}", param_count));
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $");
        param_count += 1;
        query.push_str(&param_count.to_string());

        // Execute with parameters
        let mut sql_query = sqlx::query_as::<_, Sample>(&query).bind(org_id);

        if let Some(patient_id) = filter.patient_id {
            sql_query = sql_query.bind(patient_id);
        }
        if let Some(status) = filter.sample_status {
            sql_query = sql_query.bind(status);
        }
        if let Some(is_rejected) = filter.is_rejected {
            sql_query = sql_query.bind(is_rejected);
        }

        sql_query = sql_query.bind(limit);

        let samples = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(samples)
    }

    /// Update sample status
    pub async fn update_status(&self, input: UpdateSampleStatusInput) -> Result<Sample> {
        let sample = sqlx::query_as::<_, Sample>(
            r#"
            UPDATE sample
            SET sample_status = $1, notes = COALESCE($2, notes), updated_by = $3, updated_at = NOW()
            WHERE id = $4 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.new_status)
        .bind(&input.notes)
        .bind(input.updated_by)
        .bind(input.sample_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Receive sample at lab
    pub async fn receive_sample(&self, input: ReceiveSampleInput) -> Result<Sample> {
        let sample = sqlx::query_as::<_, Sample>(
            r#"
            UPDATE sample
            SET
                sample_status = 'RECEIVED',
                received_date_time = NOW(),
                received_by = $1,
                reception_temperature = $2,
                reception_condition = $3,
                volume_ml = COALESCE($4, volume_ml),
                appearance = $5,
                is_hemolyzed = $6,
                is_lipemic = $7,
                is_icteric = $8,
                updated_at = NOW()
            WHERE id = $9 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.received_by)
        .bind(input.reception_temperature)
        .bind(&input.reception_condition)
        .bind(input.volume_ml)
        .bind(&input.appearance)
        .bind(input.is_hemolyzed)
        .bind(input.is_lipemic)
        .bind(input.is_icteric)
        .bind(input.sample_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Reject sample
    pub async fn reject_sample(&self, input: RejectSampleInput) -> Result<Sample> {
        let sample = sqlx::query_as::<_, Sample>(
            r#"
            UPDATE sample
            SET
                sample_status = 'REJECTED',
                is_rejected = TRUE,
                rejection_reason = $1,
                rejection_notes = $2,
                rejected_by = $3,
                rejected_at = NOW(),
                updated_at = NOW()
            WHERE id = $4 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.rejection_reason)
        .bind(&input.rejection_notes)
        .bind(input.rejected_by)
        .bind(input.sample_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Generate barcode for sample
    pub async fn generate_barcode(&self, sample_id: Uuid, format: &str) -> Result<Sample> {
        let sample = sqlx::query_as::<_, Sample>(
            r#"
            UPDATE sample
            SET barcode = CONCAT('BAR-', sample_id), barcode_format = $1, updated_at = NOW()
            WHERE id = $2 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(format)
        .bind(sample_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sample)
    }

    /// Get samples pending collection
    pub async fn get_pending_collection(&self, org_id: Uuid, limit: i64) -> Result<Vec<Sample>> {
        let samples = sqlx::query_as::<_, Sample>(
            r#"
            SELECT * FROM sample
            WHERE organization_id = $1
              AND sample_status = 'PENDING'
              AND is_deleted = FALSE
            ORDER BY priority DESC, created_at ASC
            LIMIT $2
            "#
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(samples)
    }

    /// Get samples by status
    pub async fn get_by_status(&self, status: SampleStatus, org_id: Uuid, limit: i64) -> Result<Vec<Sample>> {
        let samples = sqlx::query_as::<_, Sample>(
            r#"
            SELECT * FROM sample
            WHERE organization_id = $1
              AND sample_status = $2
              AND is_deleted = FALSE
            ORDER BY priority DESC, created_at DESC
            LIMIT $3
            "#
        )
        .bind(org_id)
        .bind(status)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(samples)
    }

    // ========================================================================
    // Helper methods
    // ========================================================================

    async fn generate_sample_id(&self, org_id: &Uuid, sample_type: &common::types::SampleType) -> Result<String> {
        // Get organization code
        let org_code = "LAB";  // Default, should fetch from organization service

        // Get sample type code
        let type_code = match sample_type {
            common::types::SampleType::Serum => "SER",
            common::types::SampleType::Plasma => "PLS",
            common::types::SampleType::WholeBlood => "BLD",
            common::types::SampleType::Urine => "URN",
            common::types::SampleType::Stool => "STL",
            common::types::SampleType::Csf => "CSF",
            common::types::SampleType::SynovialFluid => "SYN",
            common::types::SampleType::PleuralFluid => "PLR",
            common::types::SampleType::Sputum => "SPT",
            common::types::SampleType::Tissue => "TIS",
            common::types::SampleType::Swab => "SWB",
            common::types::SampleType::Biopsy => "BIO",
            common::types::SampleType::Aspirate => "ASP",
            common::types::SampleType::Other => "OTH",
        };

        // Get next sequence number
        let row = sqlx::query("SELECT nextval('sample_sequence')")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let sequence: i64 = row.try_get(0).map_err(|e| Error::Database(sqlx::Error::Decode(Box::new(e))))?;

        // Generate base ID
        let base_id = format!(
            "{}-{}-{}-{:06}",
            org_code,
            type_code,
            chrono::Utc::now().format("%Y%m%d"),
            sequence
        );

        // Calculate checksum (simplified Luhn)
        let checksum = common::utils::calculate_luhn_check_digit(&base_id);

        Ok(format!("{}{}", base_id, checksum))
    }
}

// ============================================================================
// Sample Aliquot Repository
// ============================================================================

#[derive(Clone)]
pub struct SampleAliquotRepository {
    pool: PgPool,
}

impl SampleAliquotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateAliquotInput, user_id: Uuid) -> Result<SampleAliquot> {
        let aliquot_id = format!("ALQ-{}-{:03}", input.parent_sample_id, input.aliquot_number);

        let aliquot = sqlx::query_as::<_, SampleAliquot>(
            r#"
            INSERT INTO sample_aliquot (
                id, parent_sample_id, aliquot_id, aliquot_number,
                volume_ml, storage_location, storage_condition, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'AVAILABLE')
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(input.parent_sample_id)
        .bind(&aliquot_id)
        .bind(input.aliquot_number)
        .bind(input.volume_ml)
        .bind(&input.storage_location)
        .bind(&input.storage_condition)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(aliquot)
    }

    pub async fn find_by_sample(&self, sample_id: Uuid) -> Result<Vec<SampleAliquot>> {
        let aliquots = sqlx::query_as::<_, SampleAliquot>(
            "SELECT * FROM sample_aliquot WHERE parent_sample_id = $1 ORDER BY aliquot_number"
        )
        .bind(sample_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(aliquots)
    }
}

// ============================================================================
// Sample Routing Repository
// ============================================================================

#[derive(Clone)]
pub struct SampleRoutingRepository {
    pool: PgPool,
}

impl SampleRoutingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RouteSampleInput) -> Result<SampleRouting> {
        let mut routing = SampleRouting {
            id: Uuid::new_v4(),
            sample_id: input.sample_id,
            route_to: input.route_to,
            routed_for: input.routed_for,
            assigned_to: input.assigned_to,
            assignment_type: input.assignment_type,
            priority: input.priority,
            routed_at: Utc::now(),
            expected_completion_time: None,
            actual_completion_time: None,
            routing_status: "PENDING".to_string(),
            is_automated: input.is_automated,
            automation_confidence: input.automation_confidence,
            routing_notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        routing.calculate_expected_completion();

        let routing = sqlx::query_as::<_, SampleRouting>(
            r#"
            INSERT INTO sample_routing (
                id, sample_id, route_to, routed_for, assigned_to, assignment_type,
                priority, routed_at, expected_completion_time, routing_status,
                is_automated, automation_confidence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(routing.id)
        .bind(routing.sample_id)
        .bind(&routing.route_to)
        .bind(&routing.routed_for)
        .bind(routing.assigned_to)
        .bind(&routing.assignment_type)
        .bind(&routing.priority)
        .bind(routing.routed_at)
        .bind(routing.expected_completion_time)
        .bind(&routing.routing_status)
        .bind(routing.is_automated)
        .bind(routing.automation_confidence)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(routing)
    }

    pub async fn find_by_sample(&self, sample_id: Uuid) -> Result<Vec<SampleRouting>> {
        let routings = sqlx::query_as::<_, SampleRouting>(
            "SELECT * FROM sample_routing WHERE sample_id = $1 ORDER BY routed_at DESC"
        )
        .bind(sample_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(routings)
    }

    pub async fn get_pending_routings(&self, org_id: Uuid, limit: i64) -> Result<Vec<SampleRouting>> {
        let routings = sqlx::query_as::<_, SampleRouting>(
            r#"
            SELECT sr.* FROM sample_routing sr
            JOIN sample s ON sr.sample_id = s.id
            WHERE s.organization_id = $1
              AND sr.routing_status = 'PENDING'
            ORDER BY sr.priority DESC, sr.routed_at ASC
            LIMIT $2
            "#
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(routings)
    }
}
