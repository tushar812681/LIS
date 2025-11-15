use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{CreatePatientInput, Patient, UpdatePatientInput};
use crate::repository::PatientRepository;
use crate::service::PatientService;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get patient by ID
    async fn patient(&self, ctx: &Context<'_>, id: String) -> Result<Patient> {
        let patient_id = Uuid::parse_str(&id)?;

        // Get database pool from context
        let pool = ctx.data::<PgPool>()?;
        let repository = PatientRepository::new(pool.clone());
        let service = PatientService::new(repository);

        let patient = service.get_patient(patient_id).await?;
        Ok(patient)
    }

    /// Get patient by MRN number
    async fn patient_by_mrn(&self, ctx: &Context<'_>, mrn_number: String) -> Result<Patient> {
        let pool = ctx.data::<PgPool>()?;
        let repository = PatientRepository::new(pool.clone());
        let service = PatientService::new(repository);

        let patient = service.get_by_mrn(&mrn_number).await?;
        Ok(patient)
    }

    /// Get patient by mobile number
    async fn patient_by_mobile(&self, ctx: &Context<'_>, mobile_number: String) -> Result<Option<Patient>> {
        let pool = ctx.data::<PgPool>()?;
        let repository = PatientRepository::new(pool.clone());

        let patient = repository.find_by_mobile(&mobile_number).await?;
        Ok(patient)
    }

    /// Search patients by query (MRN, name, mobile)
    async fn search_patients(
        &self,
        ctx: &Context<'_>,
        query: String,
        organization_id: String,
        limit: Option<i32>,
    ) -> Result<Vec<Patient>> {
        let limit = limit.unwrap_or(10).min(100) as i64;
        let org_id = Uuid::parse_str(&organization_id)?;

        let pool = ctx.data::<PgPool>()?;
        let repository = PatientRepository::new(pool.clone());
        let service = PatientService::new(repository);

        let patients = service.search_patients(&query, org_id, limit).await?;
        Ok(patients)
    }

    /// Get all patients for an organization (paginated)
    async fn patients(
        &self,
        ctx: &Context<'_>,
        organization_id: String,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Vec<Patient>> {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(20).min(100);
        let offset = (page - 1) * page_size;
        let org_id = Uuid::parse_str(&organization_id)?;

        let pool = ctx.data::<PgPool>()?;

        let patients = sqlx::query_as::<_, Patient>(
            r#"
            SELECT *
            FROM patient
            WHERE organization_id = $1 AND is_deleted = false
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(org_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

        Ok(patients)
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new patient
    async fn create_patient(
        &self,
        ctx: &Context<'_>,
        input: CreatePatientInput,
        organization_id: String,
        created_by: String,
    ) -> Result<Patient> {
        let org_id = Uuid::parse_str(&organization_id)?;
        let user_id = Uuid::parse_str(&created_by)?;

        let pool = ctx.data::<PgPool>()?;
        let repository = PatientRepository::new(pool.clone());
        let service = PatientService::new(repository);

        let patient = service.create_patient(input, org_id, user_id).await?;

        // TODO: Publish PATIENT_CREATED event to Kafka
        // TODO: Cache patient in Redis

        tracing::info!("Patient created: {} (MRN: {})", patient.id, patient.mrn_number);
        Ok(patient)
    }

    /// Update patient information
    async fn update_patient(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdatePatientInput,
        updated_by: String,
    ) -> Result<Patient> {
        let patient_id = Uuid::parse_str(&id)?;
        let user_id = Uuid::parse_str(&updated_by)?;

        let pool = ctx.data::<PgPool>()?;

        // Build update query dynamically based on provided fields
        let mut update_parts = Vec::new();
        let mut bind_index = 1;

        let mut query = String::from("UPDATE patient SET ");

        if input.first_name.is_some() {
            update_parts.push(format!("first_name = ${}", bind_index));
            bind_index += 1;
        }
        if input.mobile_number.is_some() {
            update_parts.push(format!("mobile_number = ${}", bind_index));
            bind_index += 1;
        }
        if input.email.is_some() {
            update_parts.push(format!("email = ${}", bind_index));
            bind_index += 1;
        }

        update_parts.push(format!("updated_by = ${}", bind_index));
        bind_index += 1;
        update_parts.push("updated_at = NOW()".to_string());

        query.push_str(&update_parts.join(", "));
        query.push_str(&format!(" WHERE id = ${} AND is_deleted = false RETURNING *", bind_index));

        let mut db_query = sqlx::query_as::<_, Patient>(&query);

        if let Some(first_name) = input.first_name {
            db_query = db_query.bind(first_name);
        }
        if let Some(mobile_number) = input.mobile_number {
            db_query = db_query.bind(common::utils::format_indian_phone(&mobile_number));
        }
        if let Some(email) = input.email {
            db_query = db_query.bind(email);
        }

        db_query = db_query.bind(user_id);
        db_query = db_query.bind(patient_id);

        let patient = db_query.fetch_one(pool).await?;

        // TODO: Publish PATIENT_UPDATED event
        // TODO: Invalidate cache

        tracing::info!("Patient updated: {}", patient.id);
        Ok(patient)
    }

    /// Soft delete a patient
    async fn delete_patient(
        &self,
        ctx: &Context<'_>,
        id: String,
        deleted_by: String,
    ) -> Result<bool> {
        let patient_id = Uuid::parse_str(&id)?;
        let user_id = Uuid::parse_str(&deleted_by)?;

        let pool = ctx.data::<PgPool>()?;

        let result = sqlx::query(
            r#"
            UPDATE patient
            SET is_deleted = true,
                deleted_at = NOW(),
                updated_by = $2,
                updated_at = NOW()
            WHERE id = $1 AND is_deleted = false
            "#,
        )
        .bind(patient_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            tracing::info!("Patient soft-deleted: {}", patient_id);
            // TODO: Publish PATIENT_DELETED event
            // TODO: Remove from cache
        }

        Ok(deleted)
    }
}
