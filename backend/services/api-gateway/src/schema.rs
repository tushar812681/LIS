use async_graphql::{Context, Object, Result, Schema};
use serde_json::Value;

use crate::patient_client::PatientClient;

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

/// Root Query
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get list of patients
    async fn patients(
        &self,
        ctx: &Context<'_>,
        organization_id: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        patient_client
            .get_patients(organization_id, page, page_size)
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Get single patient by ID
    async fn patient(&self, ctx: &Context<'_>, id: String) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        patient_client
            .get_patient(id)
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Get patient by MRN number
    async fn patient_by_mrn(&self, ctx: &Context<'_>, mrn_number: String) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        let query = r#"
            query GetPatientByMrn($mrnNumber: String!) {
                patientByMrn(mrnNumber: $mrnNumber) {
                    id
                    mrnNumber
                    firstName
                    lastName
                    dateOfBirth
                    gender
                    mobileNumber
                    email
                }
            }
        "#;

        let variables = serde_json::json!({ "mrnNumber": mrn_number });

        patient_client
            .execute(query, Some(variables))
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Search patients
    async fn search_patients(
        &self,
        ctx: &Context<'_>,
        query: String,
        organization_id: Option<String>,
        limit: Option<i32>,
    ) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        patient_client
            .search_patients(query, organization_id, limit)
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Health check - returns API Gateway status
    async fn health(&self) -> Result<String> {
        Ok("API Gateway is healthy".to_string())
    }
}

/// Root Mutation
#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new patient
    async fn create_patient(
        &self,
        ctx: &Context<'_>,
        input: Value,
        organization_id: String,
        created_by: String,
    ) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        patient_client
            .create_patient(input, organization_id, created_by)
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Update an existing patient
    async fn update_patient(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: Value,
        updated_by: String,
    ) -> Result<Value> {
        let patient_client = ctx.data::<PatientClient>()?;

        patient_client
            .update_patient(id, input, updated_by)
            .await
            .map_err(|e| async_graphql::Error::new(e))
    }

    /// Delete a patient (soft delete)
    async fn delete_patient(
        &self,
        ctx: &Context<'_>,
        id: String,
        deleted_by: String,
    ) -> Result<bool> {
        let patient_client = ctx.data::<PatientClient>()?;

        let query = r#"
            mutation DeletePatient($id: String!, $deletedBy: String!) {
                deletePatient(id: $id, deletedBy: $deletedBy)
            }
        "#;

        let variables = serde_json::json!({
            "id": id,
            "deletedBy": deleted_by
        });

        patient_client
            .execute::<serde_json::Value>(query, Some(variables))
            .await
            .map(|_| true)
            .map_err(|e| async_graphql::Error::new(e))
    }
}
