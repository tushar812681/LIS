use uuid::Uuid;
use common::error::{Error, Result};
use common::types::SampleStatus;

use crate::domain::*;
use crate::repository::*;

// ============================================================================
// Sample Service - Business Logic Layer
// ============================================================================

#[derive(Clone)]
pub struct SampleService {
    sample_repo: SampleRepository,
    aliquot_repo: SampleAliquotRepository,
    routing_repo: SampleRoutingRepository,
    // Event bus will be added later
    // event_bus: EventBus,
    // Cache will be added later
    // cache: CacheClient,
}

impl SampleService {
    pub fn new(
        sample_repo: SampleRepository,
        aliquot_repo: SampleAliquotRepository,
        routing_repo: SampleRoutingRepository,
    ) -> Self {
        Self {
            sample_repo,
            aliquot_repo,
            routing_repo,
        }
    }

    // ========================================================================
    // Sample Operations
    // ========================================================================

    /// Create a new sample
    pub async fn create_sample(
        &self,
        input: CreateSampleInput,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Sample> {
        // Validate input
        input.validate()?;

        // Check if order exists (would call order service in production)
        // let order = order_service.get_order(input.order_id).await?;

        // Create sample
        let mut sample = self.sample_repo.create(input, org_id, user_id).await?;

        // Generate barcode
        sample = self.sample_repo.generate_barcode(sample.id, "CODE128").await?;

        // TODO: Publish SAMPLE_CREATED event
        // self.event_bus.publish("sample.created", sample).await?;

        // TODO: Cache sample
        // self.cache.set_json(&keys::sample(sample.id), &sample).await?;

        tracing::info!("Sample created: {}", sample.sample_id);

        Ok(sample)
    }

    /// Get sample by ID
    pub async fn get_sample(&self, sample_id: Uuid) -> Result<Sample> {
        // TODO: Check cache first
        // if let Some(cached) = self.cache.get_json(&keys::sample(sample_id)).await? {
        //     return Ok(cached);
        // }

        let sample = self
            .sample_repo
            .find_by_id(sample_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Sample not found: {}", sample_id)))?;

        // TODO: Cache the result
        // self.cache.set_json_with_expiry(&keys::sample(sample_id), &sample, 300).await?;

        Ok(sample)
    }

    /// Get sample by sample ID
    pub async fn get_sample_by_sample_id(&self, sample_id: &str) -> Result<Sample> {
        let sample = self
            .sample_repo
            .find_by_sample_id(sample_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Sample not found: {}", sample_id)))?;

        Ok(sample)
    }

    /// Get sample by barcode
    pub async fn get_sample_by_barcode(&self, barcode: &str) -> Result<Sample> {
        let sample = self
            .sample_repo
            .find_by_barcode(barcode)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Sample not found with barcode: {}", barcode)))?;

        Ok(sample)
    }

    /// Get samples by patient
    pub async fn get_samples_by_patient(&self, patient_id: Uuid, limit: i64) -> Result<Vec<Sample>> {
        self.sample_repo.find_by_patient(patient_id, limit).await
    }

    /// Get samples by order
    pub async fn get_samples_by_order(&self, order_id: Uuid) -> Result<Vec<Sample>> {
        self.sample_repo.find_by_order(order_id).await
    }

    /// Search samples with filters
    pub async fn search_samples(
        &self,
        filter: SampleFilter,
        org_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Sample>> {
        self.sample_repo.search(filter, org_id, limit).await
    }

    // ========================================================================
    // Sample Workflow Operations
    // ========================================================================

    /// Update sample status
    pub async fn update_status(&self, input: UpdateSampleStatusInput) -> Result<Sample> {
        let mut sample = self.get_sample(input.sample_id).await?;

        // Business rule: validate status transition
        self.validate_status_transition(&sample.sample_status, &input.new_status)?;

        // Update status
        sample = self.sample_repo.update_status(input).await?;

        // TODO: Publish STATUS_CHANGED event
        // self.event_bus.publish("sample.status_changed", &sample).await?;

        // TODO: Invalidate cache
        // self.cache.delete(&keys::sample(sample.id)).await?;

        tracing::info!("Sample status updated: {} -> {:?}", sample.sample_id, sample.sample_status);

        Ok(sample)
    }

    /// Receive sample at laboratory
    pub async fn receive_sample(&self, input: ReceiveSampleInput) -> Result<Sample> {
        let sample = self.get_sample(input.sample_id).await?;

        // Business rule: can only receive samples that are collected or pending
        if sample.sample_status != SampleStatus::Collected
            && sample.sample_status != SampleStatus::Pending
        {
            return Err(Error::InvalidSampleStatus(
                "Sample must be collected or pending to be received".to_string()
            ));
        }

        // Receive sample
        let sample = self.sample_repo.receive_sample(input).await?;

        // Check quality and auto-accept/reject
        let sample = self.evaluate_sample_quality(sample).await?;

        // TODO: Publish SAMPLE_RECEIVED event
        // self.event_bus.publish("sample.received", &sample).await?;

        tracing::info!("Sample received: {}", sample.sample_id);

        Ok(sample)
    }

    /// Reject sample
    pub async fn reject_sample(&self, input: RejectSampleInput) -> Result<Sample> {
        let sample = self.get_sample(input.sample_id).await?;

        // Business rule: cannot reject already tested samples
        if sample.sample_status == SampleStatus::Tested {
            return Err(Error::InvalidSampleStatus(
                "Cannot reject tested samples".to_string()
            ));
        }

        // Reject sample
        let sample = self.sample_repo.reject_sample(input).await?;

        // TODO: Publish SAMPLE_REJECTED event
        // self.event_bus.publish("sample.rejected", &sample).await?;

        // TODO: Notify stakeholders
        // self.notification_service.notify_sample_rejection(&sample).await?;

        tracing::warn!("Sample rejected: {} - {}", sample.sample_id, sample.rejection_reason.as_ref().unwrap_or(&"Unknown".to_string()));

        Ok(sample)
    }

    /// Accept sample for processing
    pub async fn accept_sample(&self, sample_id: Uuid, user_id: Uuid) -> Result<Sample> {
        let sample = self.get_sample(sample_id).await?;

        // Business rule: only received samples can be made available
        if sample.sample_status != SampleStatus::Received {
            return Err(Error::InvalidSampleStatus(
                "Only received samples can be made available".to_string()
            ));
        }

        // Check if sample is acceptable
        if !sample.is_acceptable() {
            return Err(Error::InvalidSampleQuality(
                "Sample quality does not meet acceptance criteria".to_string()
            ));
        }

        let input = UpdateSampleStatusInput {
            sample_id,
            new_status: SampleStatus::Available,
            notes: Some("Sample accepted and available for processing".to_string()),
            updated_by: user_id,
        };

        self.update_status(input).await
    }

    // ========================================================================
    // Sample Routing Operations
    // ========================================================================

    /// Route sample to department/equipment
    pub async fn route_sample(&self, input: RouteSampleInput) -> Result<SampleRouting> {
        let sample = self.get_sample(input.sample_id).await?;

        // Business rule: only available samples can be routed
        if sample.sample_status != SampleStatus::Available {
            return Err(Error::InvalidSampleStatus(
                "Only available samples can be routed".to_string()
            ));
        }

        // Create routing
        let routing = self.routing_repo.create(input).await?;

        // Update sample status to IN_PROGRESS
        let status_input = UpdateSampleStatusInput {
            sample_id: sample.id,
            new_status: SampleStatus::InProgress,
            notes: Some(format!("Routed to {}", routing.route_to)),
            updated_by: Uuid::nil(),  // System action
        };
        self.update_status(status_input).await?;

        // TODO: Publish SAMPLE_ROUTED event
        // self.event_bus.publish("sample.routed", &routing).await?;

        tracing::info!("Sample routed: {} -> {}", sample.sample_id, routing.route_to);

        Ok(routing)
    }

    /// Auto-route sample based on ML model
    pub async fn auto_route_sample(&self, sample_id: Uuid) -> Result<SampleRouting> {
        let sample = self.get_sample(sample_id).await?;

        // TODO: Call ML model to determine routing
        // let routing_decision = ml_service.predict_routing(&sample).await?;

        // Mock routing decision
        let route_to = match sample.sample_type {
            common::types::SampleType::WholeBlood | common::types::SampleType::Serum => "Hematology",
            common::types::SampleType::Urine => "Urinalysis",
            _ => "General",
        }.to_string();

        let input = RouteSampleInput {
            sample_id,
            route_to,
            routed_for: "Automated routing".to_string(),
            priority: sample.priority,
            assigned_to: None,
            assignment_type: Some("AUTOMATED".to_string()),
            is_automated: true,
            automation_confidence: Some(0.95),  // Mock confidence
        };

        self.route_sample(input).await
    }

    /// Get routing history for sample
    pub async fn get_sample_routing_history(&self, sample_id: Uuid) -> Result<Vec<SampleRouting>> {
        self.routing_repo.find_by_sample(sample_id).await
    }

    // ========================================================================
    // Sample Aliquot Operations
    // ========================================================================

    /// Create aliquot from sample
    pub async fn create_aliquot(&self, input: CreateAliquotInput, user_id: Uuid) -> Result<SampleAliquot> {
        let sample = self.get_sample(input.parent_sample_id).await?;

        // Business rule: can only create aliquots from available or received samples
        if sample.sample_status != SampleStatus::Available
            && sample.sample_status != SampleStatus::Received {
            return Err(Error::InvalidSampleStatus(
                "Can only create aliquots from available or received samples".to_string()
            ));
        }

        // Check if enough volume
        if let Some(volume) = sample.volume_ml {
            if volume < input.volume_ml {
                return Err(Error::InsufficientVolume(
                    format!("Requested {} ml but only {} ml available", input.volume_ml, volume)
                ));
            }
        }

        let aliquot = self.aliquot_repo.create(input, user_id).await?;

        // TODO: Publish ALIQUOT_CREATED event
        // self.event_bus.publish("sample.aliquot_created", &aliquot).await?;

        tracing::info!("Aliquot created: {} from {}", aliquot.aliquot_id, sample.sample_id);

        Ok(aliquot)
    }

    /// Get aliquots for sample
    pub async fn get_sample_aliquots(&self, sample_id: Uuid) -> Result<Vec<SampleAliquot>> {
        self.aliquot_repo.find_by_sample(sample_id).await
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Validate status transition
    fn validate_status_transition(&self, current: &SampleStatus, new: &SampleStatus) -> Result<()> {
        let valid = match (current, new) {
            // From PENDING
            (SampleStatus::Pending, SampleStatus::Collected) => true,
            (SampleStatus::Pending, SampleStatus::Rejected) => true,

            // From COLLECTED
            (SampleStatus::Collected, SampleStatus::Received) => true,
            (SampleStatus::Collected, SampleStatus::Rejected) => true,

            // From RECEIVED
            (SampleStatus::Received, SampleStatus::Available) => true,
            (SampleStatus::Received, SampleStatus::Rejected) => true,

            // From AVAILABLE
            (SampleStatus::Available, SampleStatus::Processing) => true,
            (SampleStatus::Available, SampleStatus::InProgress) => true,
            (SampleStatus::Available, SampleStatus::Rejected) => true,

            // From PROCESSING
            (SampleStatus::Processing, SampleStatus::InProgress) => true,
            (SampleStatus::Processing, SampleStatus::Tested) => true,
            (SampleStatus::Processing, SampleStatus::Available) => true,

            // From IN_PROGRESS
            (SampleStatus::InProgress, SampleStatus::Tested) => true,
            (SampleStatus::InProgress, SampleStatus::Processing) => true,

            // From TESTED
            (SampleStatus::Tested, SampleStatus::Disposed) => true,

            _ => false,
        };

        if !valid {
            return Err(Error::InvalidStatusTransition(
                format!("Invalid transition from {:?} to {:?}", current, new)
            ));
        }

        Ok(())
    }

    /// Evaluate sample quality and auto-accept/reject
    async fn evaluate_sample_quality(&self, mut sample: Sample) -> Result<Sample> {
        let mut should_reject = false;
        let mut rejection_reason = String::new();

        // Check volume
        if let Some(vol) = sample.volume_ml {
            if vol < 0.5 {  // Minimum 0.5 ml required
                should_reject = true;
                rejection_reason = "Insufficient volume".to_string();
            }
        }

        // Check hemolysis
        if sample.is_hemolyzed {
            should_reject = true;
            rejection_reason = "Hemolyzed sample".to_string();
        }

        if should_reject {
            let reject_input = RejectSampleInput {
                sample_id: sample.id,
                rejection_reason: "INSUFFICIENT_VOLUME".to_string(),
                rejection_notes: Some(rejection_reason),
                rejected_by: Uuid::nil(),  // System rejection
            };
            return self.sample_repo.reject_sample(reject_input).await;
        }

        Ok(sample)
    }
}
