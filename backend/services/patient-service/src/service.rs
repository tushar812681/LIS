use uuid::Uuid;

use crate::domain::{CreatePatientInput, Patient, PotentialDuplicate};
use crate::repository::PatientRepository;
use common::error::Result;

pub struct PatientService {
    repository: PatientRepository,
}

impl PatientService {
    pub fn new(repository: PatientRepository) -> Self {
        Self { repository }
    }

    pub async fn create_patient(
        &self,
        input: CreatePatientInput,
        organization_id: Uuid,
        created_by: Uuid,
    ) -> Result<Patient> {
        // Check for duplicates before creating
        let duplicates = self.detect_duplicates(&input, organization_id).await?;

        if !duplicates.is_empty() {
            // Log potential duplicates but continue (can be configured differently)
            tracing::warn!("Found {} potential duplicates", duplicates.len());
        }

        self.repository.create(input, organization_id, created_by).await
    }

    pub async fn get_patient(&self, id: Uuid) -> Result<Patient> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_mrn(&self, mrn_number: &str) -> Result<Patient> {
        self.repository.find_by_mrn(mrn_number).await
    }

    pub async fn search_patients(
        &self,
        query: &str,
        organization_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Patient>> {
        self.repository.search(query, organization_id, limit).await
    }

    pub async fn detect_duplicates(
        &self,
        input: &CreatePatientInput,
        organization_id: Uuid,
    ) -> Result<Vec<PotentialDuplicate>> {
        let mut duplicates = Vec::new();

        // Check for exact mobile number match
        if let Some(existing) = self.repository.find_by_mobile(&input.mobile_number).await? {
            duplicates.push(PotentialDuplicate {
                patient_id: existing.id,
                confidence: 0.95,
                match_reason: "Exact mobile number match".to_string(),
            });
        }

        // In production, would also check:
        // - Fuzzy name matching (Jaro-Winkler distance)
        // - DOB + name combination
        // - Aadhaar number if provided

        Ok(duplicates)
    }
}
