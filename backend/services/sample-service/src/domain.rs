use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use common::types::{Priority, SampleType, SampleStatus};

// ============================================================================
// Sample Domain Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sample {
    pub id: Uuid,
    pub sample_id: String,

    // References
    pub patient_id: Uuid,
    pub order_id: Uuid,
    pub organization_id: Uuid,

    // Sample details
    #[sqlx(rename = "specimen_type")]
    pub sample_type: SampleType,
    pub sample_status: SampleStatus,
    pub priority: Priority,

    // Collection
    pub collection_date_time: Option<DateTime<Utc>>,
    pub collector_id: Option<Uuid>,
    pub collection_site: Option<String>,
    pub collection_method: Option<String>,
    pub collection_notes: Option<String>,

    // Reception
    pub received_date_time: Option<DateTime<Utc>>,
    pub received_by: Option<Uuid>,
    pub reception_temperature: Option<f64>,
    pub reception_condition: Option<String>,

    // Quality
    pub volume_ml: Option<f64>,
    pub appearance: Option<String>,
    pub is_hemolyzed: bool,
    pub is_lipemic: bool,
    pub is_icteric: bool,

    // Rejection
    pub is_rejected: bool,
    pub rejection_reason: Option<String>,
    pub rejection_notes: Option<String>,
    pub rejected_by: Option<Uuid>,
    pub rejected_at: Option<DateTime<Utc>>,

    // Storage
    pub storage_location: Option<String>,
    pub storage_condition: Option<String>,
    pub storage_position: Option<String>,
    pub storage_temperature: Option<f64>,

    // Barcode
    pub barcode: Option<String>,
    pub barcode_format: Option<String>,

    // Processing
    pub processed_date_time: Option<DateTime<Utc>>,
    pub processing_duration_minutes: Option<i32>,

    // Disposal
    pub disposal_date_time: Option<DateTime<Utc>>,
    pub disposal_method: Option<String>,
    pub disposal_by: Option<Uuid>,

    // Chain of custody
    pub chain_of_custody: Option<serde_json::Value>,

    // Metadata
    pub notes: Option<String>,
    pub special_instructions: Option<String>,
    pub biohazard_level: Option<String>,
    pub requires_fasting: bool,
    pub fasting_hours: Option<i32>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Sample {
    /// Generate a barcode for the sample
    pub fn generate_barcode(&mut self, format: &str) {
        // Use sample_id as base for barcode
        self.barcode = Some(format!("BAR-{}", self.sample_id));
        self.barcode_format = Some(format.to_string());
    }

    /// Check if sample is acceptable for processing
    pub fn is_acceptable(&self) -> bool {
        !self.is_rejected
            && self.sample_status != SampleStatus::Rejected
            && self.volume_ml.unwrap_or(0.0) > 0.0
    }

    /// Calculate turnaround time from collection to completion
    pub fn calculate_tat_hours(&self) -> Option<f64> {
        if let (Some(collected), Some(processed)) = (self.collection_date_time, self.processed_date_time) {
            let duration = processed - collected;
            Some(duration.num_minutes() as f64 / 60.0)
        } else {
            None
        }
    }

    /// Add chain of custody entry
    pub fn add_custody_entry(&mut self, handler_id: Uuid, action: &str, location: &str) {
        let entry = serde_json::json!({
            "timestamp": Utc::now(),
            "handler_id": handler_id.to_string(),
            "action": action,
            "location": location
        });

        match &mut self.chain_of_custody {
            Some(custody) => {
                if let Some(arr) = custody.as_array_mut() {
                    arr.push(entry);
                }
            }
            None => {
                self.chain_of_custody = Some(serde_json::json!([entry]));
            }
        }
    }
}

// ============================================================================
// Sample Container
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SampleContainer {
    pub id: Uuid,
    pub sample_id: Uuid,

    pub container_type: String,
    pub container_size_ml: Option<f64>,
    pub cap_color: Option<String>,

    pub additive: Option<String>,
    pub preservative: Option<String>,
    pub anticoagulant: Option<String>,

    pub container_barcode: Option<String>,
    pub position_in_rack: Option<String>,

    pub manufacturer: Option<String>,
    pub lot_number: Option<String>,
    pub expiry_date: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Sample Aliquot
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SampleAliquot {
    pub id: Uuid,
    pub parent_sample_id: Uuid,
    pub aliquot_id: String,

    pub aliquot_number: i32,
    pub volume_ml: f64,

    pub storage_location: Option<String>,
    pub storage_condition: Option<String>,
    pub storage_position: Option<String>,

    pub status: String,  // AVAILABLE, IN_USE, DEPLETED, DISPOSED

    pub assigned_to_test_id: Option<Uuid>,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by: Option<Uuid>,

    pub disposed_at: Option<DateTime<Utc>>,
    pub disposal_method: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SampleAliquot {
    pub fn is_available(&self) -> bool {
        self.status == "AVAILABLE" && self.volume_ml > 0.0
    }

    pub fn mark_as_used(&mut self, test_id: Uuid, user_id: Uuid) {
        self.status = "IN_USE".to_string();
        self.assigned_to_test_id = Some(test_id);
        self.used_at = Some(Utc::now());
        self.used_by = Some(user_id);
    }
}

// ============================================================================
// Sample Routing
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SampleRouting {
    pub id: Uuid,
    pub sample_id: Uuid,

    pub route_to: String,
    pub routed_for: String,

    pub assigned_to: Option<Uuid>,
    pub assignment_type: Option<String>,

    pub priority: Priority,

    pub routed_at: DateTime<Utc>,
    pub expected_completion_time: Option<DateTime<Utc>>,
    pub actual_completion_time: Option<DateTime<Utc>>,

    pub routing_status: String,

    pub is_automated: bool,
    pub automation_confidence: Option<f64>,

    pub routing_notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SampleRouting {
    /// Calculate expected TAT based on priority
    pub fn calculate_expected_completion(&mut self) {
        let hours = match self.priority {
            Priority::Stat => 1,      // Immediate (<1 hour)
            Priority::Urgent => 4,    // Priority (<4 hours)
            Priority::Routine => 24,  // Normal TAT
        };

        self.expected_completion_time = Some(self.routed_at + chrono::Duration::hours(hours));
    }

    /// Check if routing is delayed
    pub fn is_delayed(&self) -> bool {
        if let Some(expected) = self.expected_completion_time {
            Utc::now() > expected && self.routing_status != "COMPLETED"
        } else {
            false
        }
    }
}

// ============================================================================
// Input DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSampleInput {
    pub patient_id: Uuid,
    pub order_id: Uuid,
    pub sample_type: SampleType,
    pub priority: Priority,
    pub collection_date_time: Option<DateTime<Utc>>,
    pub collection_site: Option<String>,
    pub collection_method: Option<String>,
    pub collection_notes: Option<String>,
    pub volume_ml: Option<f64>,
    pub requires_fasting: bool,
    pub fasting_hours: Option<i32>,
    pub special_instructions: Option<String>,
}

impl CreateSampleInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        if let Some(vol) = self.volume_ml {
            if vol <= 0.0 {
                return Err(common::error::Error::Validation(
                    "Volume must be greater than 0".to_string()
                ));
            }
        }

        if self.requires_fasting {
            if self.fasting_hours.is_none() || self.fasting_hours.unwrap() < 0 {
                return Err(common::error::Error::Validation(
                    "Fasting hours must be specified and non-negative".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSampleStatusInput {
    pub sample_id: Uuid,
    pub new_status: SampleStatus,
    pub notes: Option<String>,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveSampleInput {
    pub sample_id: Uuid,
    pub received_by: Uuid,
    pub reception_temperature: Option<f64>,
    pub reception_condition: Option<String>,
    pub volume_ml: Option<f64>,
    pub appearance: Option<String>,
    pub is_hemolyzed: bool,
    pub is_lipemic: bool,
    pub is_icteric: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectSampleInput {
    pub sample_id: Uuid,
    pub rejection_reason: String,
    pub rejection_notes: Option<String>,
    pub rejected_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSampleInput {
    pub sample_id: Uuid,
    pub route_to: String,
    pub routed_for: String,
    pub priority: Priority,
    pub assigned_to: Option<Uuid>,
    pub assignment_type: Option<String>,
    pub is_automated: bool,
    pub automation_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAliquotInput {
    pub parent_sample_id: Uuid,
    pub aliquot_number: i32,
    pub volume_ml: f64,
    pub storage_location: Option<String>,
    pub storage_condition: Option<String>,
}

// ============================================================================
// Query Filters
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleFilter {
    pub patient_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub sample_type: Option<SampleType>,
    pub sample_status: Option<SampleStatus>,
    pub priority: Option<Priority>,
    pub collection_date_from: Option<DateTime<Utc>>,
    pub collection_date_to: Option<DateTime<Utc>>,
    pub is_rejected: Option<bool>,
    pub barcode: Option<String>,
}
