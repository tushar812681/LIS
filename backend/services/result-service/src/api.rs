use async_graphql::{Context, Object, Result, SimpleObject, InputObject, Enum, ID};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::*;
use crate::service::ResultService;

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(SimpleObject)]
pub struct TestResultGQL {
    pub id: ID,
    pub result_number: String,
    pub patient_id: ID,
    pub order_id: ID,
    pub test_id: ID,
    pub sample_id: ID,
    pub test_code: String,
    pub test_name: String,
    pub department: Option<String>,
    pub result_value: Option<String>,
    pub result_unit: Option<String>,
    pub result_type: String,
    pub reference_range_text: Option<String>,
    pub interpretation: InterpretationEnum,
    pub clinical_interpretation: Option<String>,
    pub critical_flag: CriticalFlagEnum,
    pub delta_flag: DeltaFlagEnum,
    pub is_abnormal: bool,
    pub is_critical: bool,
    pub result_status: ResultStatusEnum,
    pub verification_status: VerificationStatusEnum,
    pub entry_method: Option<String>,
    pub result_date: String,
    pub reported_date: Option<String>,
    pub technician_notes: Option<String>,
    pub pathologist_notes: Option<String>,
    pub is_corrected: bool,
    pub correction_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TestResult> for TestResultGQL {
    fn from(result: TestResult) -> Self {
        Self {
            id: result.id.to_string().into(),
            result_number: result.result_number,
            patient_id: result.patient_id.to_string().into(),
            order_id: result.order_id.to_string().into(),
            test_id: result.test_id.to_string().into(),
            sample_id: result.sample_id.to_string().into(),
            test_code: result.test_code,
            test_name: result.test_name,
            department: result.department,
            result_value: result.result_value,
            result_unit: result.result_unit,
            result_type: result.result_type,
            reference_range_text: result.reference_range_text,
            interpretation: result.interpretation.into(),
            clinical_interpretation: result.clinical_interpretation,
            critical_flag: result.critical_flag.into(),
            delta_flag: result.delta_flag.into(),
            is_abnormal: result.is_abnormal,
            is_critical: result.is_critical,
            result_status: result.result_status.into(),
            verification_status: result.verification_status.into(),
            entry_method: result.entry_method,
            result_date: result.result_date.to_rfc3339(),
            reported_date: result.reported_date.map(|dt| dt.to_rfc3339()),
            technician_notes: result.technician_notes,
            pathologist_notes: result.pathologist_notes,
            is_corrected: result.is_corrected,
            correction_reason: result.correction_reason,
            created_at: result.created_at.to_rfc3339(),
            updated_at: result.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CriticalResultNotificationGQL {
    pub id: ID,
    pub result_id: ID,
    pub notified_to: String,
    pub notification_method: String,
    pub notification_date: String,
    pub acknowledged: bool,
    pub acknowledged_by: Option<String>,
    pub acknowledgment_date: Option<String>,
    pub notes: Option<String>,
}

impl From<CriticalResultNotification> for CriticalResultNotificationGQL {
    fn from(notification: CriticalResultNotification) -> Self {
        Self {
            id: notification.id.to_string().into(),
            result_id: notification.result_id.to_string().into(),
            notified_to: notification.notified_to,
            notification_method: notification.notification_method,
            notification_date: notification.notification_date.to_rfc3339(),
            acknowledged: notification.acknowledged,
            acknowledged_by: notification.acknowledged_by,
            acknowledgment_date: notification.acknowledgment_date.map(|dt| dt.to_rfc3339()),
            notes: notification.notes,
        }
    }
}

// ============================================================================
// Enums
// ============================================================================

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ResultStatusEnum {
    Pending,
    InProgress,
    Preliminary,
    Final,
    Corrected,
    Cancelled,
    Amended,
}

impl From<ResultStatus> for ResultStatusEnum {
    fn from(status: ResultStatus) -> Self {
        match status {
            ResultStatus::Pending => ResultStatusEnum::Pending,
            ResultStatus::InProgress => ResultStatusEnum::InProgress,
            ResultStatus::Preliminary => ResultStatusEnum::Preliminary,
            ResultStatus::Final => ResultStatusEnum::Final,
            ResultStatus::Corrected => ResultStatusEnum::Corrected,
            ResultStatus::Cancelled => ResultStatusEnum::Cancelled,
            ResultStatus::Amended => ResultStatusEnum::Amended,
        }
    }
}

impl From<ResultStatusEnum> for ResultStatus {
    fn from(status: ResultStatusEnum) -> Self {
        match status {
            ResultStatusEnum::Pending => ResultStatus::Pending,
            ResultStatusEnum::InProgress => ResultStatus::InProgress,
            ResultStatusEnum::Preliminary => ResultStatus::Preliminary,
            ResultStatusEnum::Final => ResultStatus::Final,
            ResultStatusEnum::Corrected => ResultStatus::Corrected,
            ResultStatusEnum::Cancelled => ResultStatus::Cancelled,
            ResultStatusEnum::Amended => ResultStatus::Amended,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VerificationStatusEnum {
    NotVerified,
    AutoVerified,
    ManuallyVerified,
    VerificationFailed,
    PendingReview,
}

impl From<VerificationStatus> for VerificationStatusEnum {
    fn from(status: VerificationStatus) -> Self {
        match status {
            VerificationStatus::NotVerified => VerificationStatusEnum::NotVerified,
            VerificationStatus::AutoVerified => VerificationStatusEnum::AutoVerified,
            VerificationStatus::ManuallyVerified => VerificationStatusEnum::ManuallyVerified,
            VerificationStatus::VerificationFailed => VerificationStatusEnum::VerificationFailed,
            VerificationStatus::PendingReview => VerificationStatusEnum::PendingReview,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum CriticalFlagEnum {
    None,
    Low,
    High,
    PanicLow,
    PanicHigh,
}

impl From<CriticalFlag> for CriticalFlagEnum {
    fn from(flag: CriticalFlag) -> Self {
        match flag {
            CriticalFlag::None => CriticalFlagEnum::None,
            CriticalFlag::Low => CriticalFlagEnum::Low,
            CriticalFlag::High => CriticalFlagEnum::High,
            CriticalFlag::PanicLow => CriticalFlagEnum::PanicLow,
            CriticalFlag::PanicHigh => CriticalFlagEnum::PanicHigh,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DeltaFlagEnum {
    Normal,
    SignificantIncrease,
    SignificantDecrease,
    NoPreviousResult,
}

impl From<DeltaFlag> for DeltaFlagEnum {
    fn from(flag: DeltaFlag) -> Self {
        match flag {
            DeltaFlag::Normal => DeltaFlagEnum::Normal,
            DeltaFlag::SignificantIncrease => DeltaFlagEnum::SignificantIncrease,
            DeltaFlag::SignificantDecrease => DeltaFlagEnum::SignificantDecrease,
            DeltaFlag::NoPreviousResult => DeltaFlagEnum::NoPreviousResult,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum InterpretationEnum {
    Normal,
    AbnormalLow,
    AbnormalHigh,
    CriticalLow,
    CriticalHigh,
    Indeterminate,
}

impl From<InterpretationType> for InterpretationEnum {
    fn from(interpretation: InterpretationType) -> Self {
        match interpretation {
            InterpretationType::Normal => InterpretationEnum::Normal,
            InterpretationType::AbnormalLow => InterpretationEnum::AbnormalLow,
            InterpretationType::AbnormalHigh => InterpretationEnum::AbnormalHigh,
            InterpretationType::CriticalLow => InterpretationEnum::CriticalLow,
            InterpretationType::CriticalHigh => InterpretationEnum::CriticalHigh,
            InterpretationType::Indeterminate => InterpretationEnum::Indeterminate,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(InputObject)]
pub struct CreateResultInputGQL {
    pub order_id: ID,
    pub order_item_id: ID,
    pub test_id: ID,
    pub sample_id: ID,
    pub result_value: String,
    pub result_unit: Option<String>,
    pub entry_method: String,
    pub instrument_id: Option<ID>,
    pub run_number: Option<String>,
    pub technician_notes: Option<String>,
}

impl TryFrom<CreateResultInputGQL> for CreateResultInput {
    type Error = String;

    fn try_from(input: CreateResultInputGQL) -> std::result::Result<Self, Self::Error> {
        let order_id = Uuid::parse_str(&input.order_id)
            .map_err(|e| format!("Invalid order_id: {}", e))?;
        let order_item_id = Uuid::parse_str(&input.order_item_id)
            .map_err(|e| format!("Invalid order_item_id: {}", e))?;
        let test_id = Uuid::parse_str(&input.test_id)
            .map_err(|e| format!("Invalid test_id: {}", e))?;
        let sample_id = Uuid::parse_str(&input.sample_id)
            .map_err(|e| format!("Invalid sample_id: {}", e))?;

        let instrument_id = if let Some(id) = input.instrument_id {
            Some(Uuid::parse_str(&id).map_err(|e| format!("Invalid instrument_id: {}", e))?)
        } else {
            None
        };

        Ok(CreateResultInput {
            order_id,
            order_item_id,
            test_id,
            sample_id,
            result_value: input.result_value,
            result_unit: input.result_unit,
            entry_method: input.entry_method,
            instrument_id,
            run_number: input.run_number,
            technician_notes: input.technician_notes,
        })
    }
}

#[derive(InputObject)]
pub struct UpdateResultInputGQL {
    pub result_id: ID,
    pub result_value: String,
    pub result_unit: Option<String>,
    pub technician_notes: Option<String>,
}

impl TryFrom<UpdateResultInputGQL> for UpdateResultInput {
    type Error = String;

    fn try_from(input: UpdateResultInputGQL) -> std::result::Result<Self, Self::Error> {
        let result_id = Uuid::parse_str(&input.result_id)
            .map_err(|e| format!("Invalid result_id: {}", e))?;

        Ok(UpdateResultInput {
            result_id,
            result_value: input.result_value,
            result_unit: input.result_unit,
            technician_notes: input.technician_notes,
        })
    }
}

#[derive(InputObject)]
pub struct VerifyResultInputGQL {
    pub result_id: ID,
    pub verification_method: String,
    pub pathologist_notes: Option<String>,
}

impl TryFrom<VerifyResultInputGQL> for VerifyResultInput {
    type Error = String;

    fn try_from(input: VerifyResultInputGQL) -> std::result::Result<Self, Self::Error> {
        let result_id = Uuid::parse_str(&input.result_id)
            .map_err(|e| format!("Invalid result_id: {}", e))?;

        Ok(VerifyResultInput {
            result_id,
            verification_method: input.verification_method,
            pathologist_notes: input.pathologist_notes,
        })
    }
}

#[derive(InputObject)]
pub struct ApproveResultInputGQL {
    pub result_id: ID,
    pub approval_notes: Option<String>,
}

impl TryFrom<ApproveResultInputGQL> for ApproveResultInput {
    type Error = String;

    fn try_from(input: ApproveResultInputGQL) -> std::result::Result<Self, Self::Error> {
        let result_id = Uuid::parse_str(&input.result_id)
            .map_err(|e| format!("Invalid result_id: {}", e))?;

        Ok(ApproveResultInput {
            result_id,
            approval_notes: input.approval_notes,
        })
    }
}

#[derive(InputObject)]
pub struct CorrectResultInputGQL {
    pub result_id: ID,
    pub new_result_value: String,
    pub correction_reason: String,
}

impl TryFrom<CorrectResultInputGQL> for CorrectResultInput {
    type Error = String;

    fn try_from(input: CorrectResultInputGQL) -> std::result::Result<Self, Self::Error> {
        let result_id = Uuid::parse_str(&input.result_id)
            .map_err(|e| format!("Invalid result_id: {}", e))?;

        Ok(CorrectResultInput {
            result_id,
            new_result_value: input.new_result_value,
            correction_reason: input.correction_reason,
        })
    }
}

#[derive(InputObject)]
pub struct RecordCriticalNotificationInputGQL {
    pub result_id: ID,
    pub notified_to: String,
    pub notification_method: String,
    pub caller_name: Option<String>,
    pub call_back_number: Option<String>,
    pub notes: Option<String>,
}

impl TryFrom<RecordCriticalNotificationInputGQL> for RecordCriticalNotificationInput {
    type Error = String;

    fn try_from(input: RecordCriticalNotificationInputGQL) -> std::result::Result<Self, Self::Error> {
        let result_id = Uuid::parse_str(&input.result_id)
            .map_err(|e| format!("Invalid result_id: {}", e))?;

        Ok(RecordCriticalNotificationInput {
            result_id,
            notified_to: input.notified_to,
            notification_method: input.notification_method,
            caller_name: input.caller_name,
            call_back_number: input.call_back_number,
            notes: input.notes,
        })
    }
}

// ============================================================================
// GraphQL Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get result by ID
    async fn result(&self, ctx: &Context<'_>, id: ID) -> Result<Option<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let result_id = Uuid::parse_str(&id)?;

        match service.get_result(result_id).await {
            Ok(result) => Ok(Some(result.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get result by result number
    async fn result_by_number(&self, ctx: &Context<'_>, result_number: String) -> Result<Option<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;

        match service.get_result_by_number(&result_number).await {
            Ok(result) => Ok(Some(result.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get results by patient
    async fn results_by_patient(&self, ctx: &Context<'_>, patient_id: ID, limit: Option<i32>) -> Result<Vec<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let id = Uuid::parse_str(&patient_id)?;
        let results = service.get_results_by_patient(id, limit.unwrap_or(50) as i64).await?;
        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get results by order
    async fn results_by_order(&self, ctx: &Context<'_>, order_id: ID) -> Result<Vec<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let id = Uuid::parse_str(&order_id)?;
        let results = service.get_results_by_order(id).await?;
        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get results by sample
    async fn results_by_sample(&self, ctx: &Context<'_>, sample_id: ID) -> Result<Vec<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let id = Uuid::parse_str(&sample_id)?;
        let results = service.get_results_by_sample(id).await?;
        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get pending verification results
    async fn pending_verification(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let org_id = Uuid::nil(); // TODO: Get from auth context
        let results = service.get_pending_verification(org_id, limit.unwrap_or(50) as i64).await?;
        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get critical results
    async fn critical_results(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<TestResultGQL>> {
        let service = ctx.data::<ResultService>()?;
        let org_id = Uuid::nil(); // TODO: Get from auth context
        let results = service.get_critical_results(org_id, limit.unwrap_or(50) as i64).await?;
        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get critical notifications for a result
    async fn critical_notifications(&self, ctx: &Context<'_>, result_id: ID) -> Result<Vec<CriticalResultNotificationGQL>> {
        let service = ctx.data::<ResultService>()?;
        let id = Uuid::parse_str(&result_id)?;
        let notifications = service.get_critical_notifications(id).await?;
        Ok(notifications.into_iter().map(|n| n.into()).collect())
    }
}

// ============================================================================
// GraphQL Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create new result
    async fn create_result(&self, ctx: &Context<'_>, input: CreateResultInputGQL) -> Result<TestResultGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get org_id and user_id from auth context
        let org_id = Uuid::nil();
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let result = service.create_result(domain_input, org_id, user_id).await?;
        Ok(result.into())
    }

    /// Update result
    async fn update_result(&self, ctx: &Context<'_>, input: UpdateResultInputGQL) -> Result<TestResultGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let result = service.update_result(domain_input, user_id).await?;
        Ok(result.into())
    }

    /// Verify result
    async fn verify_result(&self, ctx: &Context<'_>, input: VerifyResultInputGQL) -> Result<TestResultGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let result = service.verify_result(domain_input, user_id).await?;
        Ok(result.into())
    }

    /// Approve result
    async fn approve_result(&self, ctx: &Context<'_>, input: ApproveResultInputGQL) -> Result<TestResultGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let result = service.approve_result(domain_input, user_id).await?;
        Ok(result.into())
    }

    /// Correct result
    async fn correct_result(&self, ctx: &Context<'_>, input: CorrectResultInputGQL) -> Result<TestResultGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let result = service.correct_result(domain_input, user_id).await?;
        Ok(result.into())
    }

    /// Record critical notification
    async fn record_critical_notification(&self, ctx: &Context<'_>, input: RecordCriticalNotificationInputGQL) -> Result<CriticalResultNotificationGQL> {
        let service = ctx.data::<ResultService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let notification = service.record_critical_notification(domain_input, user_id).await?;
        Ok(notification.into())
    }

    /// Acknowledge critical notification
    async fn acknowledge_critical_notification(
        &self,
        ctx: &Context<'_>,
        notification_id: ID,
        acknowledged_by: String,
        method: String
    ) -> Result<CriticalResultNotificationGQL> {
        let service = ctx.data::<ResultService>()?;
        let id = Uuid::parse_str(&notification_id)?;

        let notification = service.acknowledge_critical_notification(id, &acknowledged_by, &method).await?;
        Ok(notification.into())
    }
}
