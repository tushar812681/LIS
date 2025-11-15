use async_graphql::{Context, Object, Result, ID, SimpleObject, InputObject, Enum};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use common::types::{SampleType, SampleStatus, Priority};

use crate::domain::*;
use crate::service::SampleService;

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(SimpleObject)]
pub struct SampleGQL {
    pub id: ID,
    pub sample_id: String,
    pub patient_id: ID,
    pub order_id: ID,
    pub organization_id: ID,

    pub sample_type: SampleTypeEnum,
    pub sample_status: SampleStatusEnum,
    pub priority: PriorityEnum,

    pub collection_date_time: Option<DateTime<Utc>>,
    pub collection_site: Option<String>,
    pub collection_method: Option<String>,
    pub collection_notes: Option<String>,

    pub received_date_time: Option<DateTime<Utc>>,
    pub reception_temperature: Option<f64>,
    pub reception_condition: Option<String>,

    pub volume_ml: Option<f64>,
    pub appearance: Option<String>,
    pub is_hemolyzed: bool,
    pub is_lipemic: bool,
    pub is_icteric: bool,

    pub is_rejected: bool,
    pub rejection_reason: Option<String>,
    pub rejection_notes: Option<String>,
    pub rejected_at: Option<DateTime<Utc>>,

    pub storage_location: Option<String>,
    pub storage_condition: Option<String>,
    pub storage_position: Option<String>,
    pub storage_temperature: Option<f64>,

    pub barcode: Option<String>,
    pub barcode_format: Option<String>,

    pub processed_date_time: Option<DateTime<Utc>>,
    pub processing_duration_minutes: Option<i32>,

    pub notes: Option<String>,
    pub special_instructions: Option<String>,
    pub biohazard_level: Option<String>,
    pub requires_fasting: bool,
    pub fasting_hours: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl From<Sample> for SampleGQL {
    fn from(sample: Sample) -> Self {
        Self {
            id: sample.id.to_string().into(),
            sample_id: sample.sample_id,
            patient_id: sample.patient_id.to_string().into(),
            order_id: sample.order_id.to_string().into(),
            organization_id: sample.organization_id.to_string().into(),
            sample_type: sample.sample_type.into(),
            sample_status: sample.sample_status.into(),
            priority: sample.priority.into(),
            collection_date_time: sample.collection_date_time,
            collection_site: sample.collection_site,
            collection_method: sample.collection_method,
            collection_notes: sample.collection_notes,
            received_date_time: sample.received_date_time,
            reception_temperature: sample.reception_temperature,
            reception_condition: sample.reception_condition,
            volume_ml: sample.volume_ml,
            appearance: sample.appearance,
            is_hemolyzed: sample.is_hemolyzed,
            is_lipemic: sample.is_lipemic,
            is_icteric: sample.is_icteric,
            is_rejected: sample.is_rejected,
            rejection_reason: sample.rejection_reason,
            rejection_notes: sample.rejection_notes,
            rejected_at: sample.rejected_at,
            storage_location: sample.storage_location,
            storage_condition: sample.storage_condition,
            storage_position: sample.storage_position,
            storage_temperature: sample.storage_temperature,
            barcode: sample.barcode,
            barcode_format: sample.barcode_format,
            processed_date_time: sample.processed_date_time,
            processing_duration_minutes: sample.processing_duration_minutes,
            notes: sample.notes,
            special_instructions: sample.special_instructions,
            biohazard_level: sample.biohazard_level,
            requires_fasting: sample.requires_fasting,
            fasting_hours: sample.fasting_hours,
            created_at: sample.created_at,
            updated_at: sample.updated_at,
            is_active: sample.is_active,
        }
    }
}

// ============================================================================
// Enums
// ============================================================================

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SampleTypeEnum {
    Serum,
    Plasma,
    WholeBlood,
    Urine,
    Stool,
    Sputum,
    Csf,
    Tissue,
    Swab,
    Biopsy,
    Aspirate,
    Other,
    SynovialFluid,
    PleuralFluid,
}

impl From<common::types::SampleType> for SampleTypeEnum {
    fn from(st: common::types::SampleType) -> Self {
        match st {
            common::types::SampleType::Serum => Self::Serum,
            common::types::SampleType::Plasma => Self::Plasma,
            common::types::SampleType::WholeBlood => Self::WholeBlood,
            common::types::SampleType::Urine => Self::Urine,
            common::types::SampleType::Stool => Self::Stool,
            common::types::SampleType::Csf => Self::Csf,
            common::types::SampleType::SynovialFluid => Self::SynovialFluid,
            common::types::SampleType::PleuralFluid => Self::PleuralFluid,
            common::types::SampleType::Sputum => Self::Sputum,
            common::types::SampleType::Tissue => Self::Tissue,
            common::types::SampleType::Swab => Self::Swab,
            common::types::SampleType::Biopsy => Self::Biopsy,
            common::types::SampleType::Aspirate => Self::Aspirate,
            common::types::SampleType::Other => Self::Other,
        }
    }
}

impl Into<common::types::SampleType> for SampleTypeEnum {
    fn into(self) -> common::types::SampleType {
        match self {
            Self::Serum => common::types::SampleType::Serum,
            Self::Plasma => common::types::SampleType::Plasma,
            Self::WholeBlood => common::types::SampleType::WholeBlood,
            Self::Urine => common::types::SampleType::Urine,
            Self::Stool => common::types::SampleType::Stool,
            Self::Csf => common::types::SampleType::Csf,
            Self::SynovialFluid => common::types::SampleType::SynovialFluid,
            Self::PleuralFluid => common::types::SampleType::PleuralFluid,
            Self::Sputum => common::types::SampleType::Sputum,
            Self::Tissue => common::types::SampleType::Tissue,
            Self::Swab => common::types::SampleType::Swab,
            Self::Biopsy => common::types::SampleType::Biopsy,
            Self::Aspirate => common::types::SampleType::Aspirate,
            Self::Other => common::types::SampleType::Other,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SampleStatusEnum {
    Pending,
    Collected,
    Received,
    Processing,
    Available,
    InProgress,
    Tested,
    Rejected,
    Disposed,
}

impl From<common::types::SampleStatus> for SampleStatusEnum {
    fn from(status: common::types::SampleStatus) -> Self {
        match status {
            common::types::SampleStatus::Pending => Self::Pending,
            common::types::SampleStatus::Collected => Self::Collected,
            common::types::SampleStatus::Received => Self::Received,
            common::types::SampleStatus::Processing => Self::Processing,
            common::types::SampleStatus::Available => Self::Available,
            common::types::SampleStatus::InProgress => Self::InProgress,
            common::types::SampleStatus::Tested => Self::Tested,
            common::types::SampleStatus::Rejected => Self::Rejected,
            common::types::SampleStatus::Disposed => Self::Disposed,
        }
    }
}

impl Into<common::types::SampleStatus> for SampleStatusEnum {
    fn into(self) -> common::types::SampleStatus {
        match self {
            Self::Pending => common::types::SampleStatus::Pending,
            Self::Collected => common::types::SampleStatus::Collected,
            Self::Received => common::types::SampleStatus::Received,
            Self::Processing => common::types::SampleStatus::Processing,
            Self::Available => common::types::SampleStatus::Available,
            Self::InProgress => common::types::SampleStatus::InProgress,
            Self::Tested => common::types::SampleStatus::Tested,
            Self::Rejected => common::types::SampleStatus::Rejected,
            Self::Disposed => common::types::SampleStatus::Disposed,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PriorityEnum {
    Stat,
    Urgent,
    Routine,
}

impl From<common::types::Priority> for PriorityEnum {
    fn from(p: common::types::Priority) -> Self {
        match p {
            common::types::Priority::Stat => Self::Stat,
            common::types::Priority::Urgent => Self::Urgent,
            common::types::Priority::Routine => Self::Routine,
        }
    }
}

impl Into<common::types::Priority> for PriorityEnum {
    fn into(self) -> common::types::Priority {
        match self {
            Self::Stat => common::types::Priority::Stat,
            Self::Urgent => common::types::Priority::Urgent,
            Self::Routine => common::types::Priority::Routine,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(InputObject)]
pub struct CreateSampleInputGQL {
    pub patient_id: ID,
    pub order_id: ID,
    pub sample_type: SampleTypeEnum,
    pub priority: PriorityEnum,
    pub collection_date_time: Option<DateTime<Utc>>,
    pub collection_site: Option<String>,
    pub collection_method: Option<String>,
    pub collection_notes: Option<String>,
    pub volume_ml: Option<f64>,
    pub requires_fasting: bool,
    pub fasting_hours: Option<i32>,
    pub special_instructions: Option<String>,
}

impl Into<CreateSampleInput> for CreateSampleInputGQL {
    fn into(self) -> CreateSampleInput {
        CreateSampleInput {
            patient_id: Uuid::parse_str(&self.patient_id).unwrap(),
            order_id: Uuid::parse_str(&self.order_id).unwrap(),
            sample_type: self.sample_type.into(),
            priority: self.priority.into(),
            collection_date_time: self.collection_date_time,
            collection_site: self.collection_site,
            collection_method: self.collection_method,
            collection_notes: self.collection_notes,
            volume_ml: self.volume_ml,
            requires_fasting: self.requires_fasting,
            fasting_hours: self.fasting_hours,
            special_instructions: self.special_instructions,
        }
    }
}

#[derive(InputObject)]
pub struct ReceiveSampleInputGQL {
    pub reception_temperature: Option<f64>,
    pub reception_condition: Option<String>,
    pub volume_ml: Option<f64>,
    pub appearance: Option<String>,
    pub is_hemolyzed: bool,
    pub is_lipemic: bool,
    pub is_icteric: bool,
}

#[derive(InputObject)]
pub struct RejectSampleInputGQL {
    pub rejection_reason: String,
    pub rejection_notes: Option<String>,
}

// ============================================================================
// Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get sample by ID
    async fn sample(&self, ctx: &Context<'_>, id: ID) -> Result<Option<SampleGQL>> {
        let service = ctx.data::<SampleService>()?;
        let sample_id = Uuid::parse_str(&id)?;

        match service.get_sample(sample_id).await {
            Ok(sample) => Ok(Some(sample.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get sample by sample ID (human-readable ID)
    async fn sample_by_sample_id(&self, ctx: &Context<'_>, sample_id: String) -> Result<Option<SampleGQL>> {
        let service = ctx.data::<SampleService>()?;

        match service.get_sample_by_sample_id(&sample_id).await {
            Ok(sample) => Ok(Some(sample.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get sample by barcode
    async fn sample_by_barcode(&self, ctx: &Context<'_>, barcode: String) -> Result<Option<SampleGQL>> {
        let service = ctx.data::<SampleService>()?;

        match service.get_sample_by_barcode(&barcode).await {
            Ok(sample) => Ok(Some(sample.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get samples by patient
    async fn samples_by_patient(
        &self,
        ctx: &Context<'_>,
        patient_id: ID,
        limit: Option<i64>,
    ) -> Result<Vec<SampleGQL>> {
        let service = ctx.data::<SampleService>()?;
        let patient_uuid = Uuid::parse_str(&patient_id)?;
        let limit = limit.unwrap_or(50);

        let samples = service.get_samples_by_patient(patient_uuid, limit).await?;
        Ok(samples.into_iter().map(|s| s.into()).collect())
    }

    /// Get samples by order
    async fn samples_by_order(&self, ctx: &Context<'_>, order_id: ID) -> Result<Vec<SampleGQL>> {
        let service = ctx.data::<SampleService>()?;
        let order_uuid = Uuid::parse_str(&order_id)?;

        let samples = service.get_samples_by_order(order_uuid).await?;
        Ok(samples.into_iter().map(|s| s.into()).collect())
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new sample
    async fn create_sample(
        &self,
        ctx: &Context<'_>,
        input: CreateSampleInputGQL,
    ) -> Result<SampleGQL> {
        let service = ctx.data::<SampleService>()?;
        // TODO: Get from authentication context
        let org_id = Uuid::nil();
        let user_id = Uuid::nil();

        let sample = service.create_sample(input.into(), org_id, user_id).await?;
        Ok(sample.into())
    }

    /// Receive sample at laboratory
    async fn receive_sample(
        &self,
        ctx: &Context<'_>,
        sample_id: ID,
        input: ReceiveSampleInputGQL,
    ) -> Result<SampleGQL> {
        let service = ctx.data::<SampleService>()?;
        let sample_uuid = Uuid::parse_str(&sample_id)?;
        let user_id = Uuid::nil(); // TODO: Get from auth context

        let receive_input = ReceiveSampleInput {
            sample_id: sample_uuid,
            received_by: user_id,
            reception_temperature: input.reception_temperature,
            reception_condition: input.reception_condition,
            volume_ml: input.volume_ml,
            appearance: input.appearance,
            is_hemolyzed: input.is_hemolyzed,
            is_lipemic: input.is_lipemic,
            is_icteric: input.is_icteric,
        };

        let sample = service.receive_sample(receive_input).await?;
        Ok(sample.into())
    }

    /// Reject sample
    async fn reject_sample(
        &self,
        ctx: &Context<'_>,
        sample_id: ID,
        input: RejectSampleInputGQL,
    ) -> Result<SampleGQL> {
        let service = ctx.data::<SampleService>()?;
        let sample_uuid = Uuid::parse_str(&sample_id)?;
        let user_id = Uuid::nil(); // TODO: Get from auth context

        let reject_input = RejectSampleInput {
            sample_id: sample_uuid,
            rejection_reason: input.rejection_reason,
            rejection_notes: input.rejection_notes,
            rejected_by: user_id,
        };

        let sample = service.reject_sample(reject_input).await?;
        Ok(sample.into())
    }

    /// Accept sample for processing
    async fn accept_sample(&self, ctx: &Context<'_>, sample_id: ID) -> Result<SampleGQL> {
        let service = ctx.data::<SampleService>()?;
        let sample_uuid = Uuid::parse_str(&sample_id)?;
        let user_id = Uuid::nil(); // TODO: Get from auth context

        let sample = service.accept_sample(sample_uuid, user_id).await?;
        Ok(sample.into())
    }

    /// Auto-route sample
    async fn auto_route_sample(&self, ctx: &Context<'_>, sample_id: ID) -> Result<bool> {
        let service = ctx.data::<SampleService>()?;
        let sample_uuid = Uuid::parse_str(&sample_id)?;

        service.auto_route_sample(sample_uuid).await?;
        Ok(true)
    }
}
