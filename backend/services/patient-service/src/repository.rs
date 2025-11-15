use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{CreatePatientInput, Patient};
use common::error::{Error, Result};

pub struct PatientRepository {
    pool: PgPool,
}

impl PatientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreatePatientInput, organization_id: Uuid, created_by: Uuid) -> Result<Patient> {
        // Validate input
        input.validate()?;

        // Generate MRN number (simplified - in production, use proper sequence)
        let mrn_number = format!("MRN{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        // Build full name
        let mut full_name_parts: Vec<&str> = vec![&input.first_name];
        if let Some(middle) = &input.middle_name {
            full_name_parts.push(middle.as_str());
        }
        if let Some(last) = &input.last_name {
            full_name_parts.push(last.as_str());
        }
        let full_name = full_name_parts.join(" ");

        // Calculate age
        let age = common::utils::calculate_age(input.date_of_birth);

        // Format phone number
        let mobile_number = common::utils::format_indian_phone(&input.mobile_number);

        let patient = sqlx::query_as::<_, Patient>(
            r#"
            INSERT INTO patient (
                id, mrn_number, organization_id,
                first_name, middle_name, last_name, full_name,
                date_of_birth, age, gender, mobile_number, alternate_mobile, email,
                aadhaar_number, preferred_language, preferred_communication,
                occupation, marital_status, nationality,
                registration_source, created_by, updated_by
            )
            VALUES (
                $1, $2, $3,
                $4, $5, $6, $7,
                $8, $9, $10, $11, $12, $13,
                $14, $15, $16,
                $17, $18, $19,
                $20, $21, $22
            )
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&mrn_number)
        .bind(organization_id)
        .bind(&input.first_name)
        .bind(&input.middle_name)
        .bind(&input.last_name)
        .bind(&full_name)
        .bind(input.date_of_birth)
        .bind(age)
        .bind(input.gender)
        .bind(&mobile_number)
        .bind(&input.alternate_mobile)
        .bind(&input.email)
        .bind(&input.aadhaar_number) // Should be encrypted in production
        .bind(input.preferred_language.unwrap_or_default())
        .bind(input.preferred_communication.unwrap_or(common::types::CommunicationChannel::WhatsApp))
        .bind(&input.occupation)
        .bind(&input.marital_status)
        .bind(input.nationality.unwrap_or_else(|| "Indian".to_string()))
        .bind(common::types::RegistrationSource::WalkIn)
        .bind(created_by)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(patient)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Patient> {
        let patient = sqlx::query_as::<_, Patient>(
            "SELECT * FROM patient WHERE id = $1 AND is_deleted = false",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound("Patient not found".to_string()))?;

        Ok(patient)
    }

    pub async fn find_by_mrn(&self, mrn_number: &str) -> Result<Patient> {
        let patient = sqlx::query_as::<_, Patient>(
            "SELECT * FROM patient WHERE mrn_number = $1 AND is_deleted = false",
        )
        .bind(mrn_number)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::NotFound("Patient not found".to_string()))?;

        Ok(patient)
    }

    pub async fn find_by_mobile(&self, mobile_number: &str) -> Result<Option<Patient>> {
        let formatted_mobile = common::utils::format_indian_phone(mobile_number);

        let patient = sqlx::query_as::<_, Patient>(
            "SELECT * FROM patient WHERE mobile_number = $1 AND is_deleted = false",
        )
        .bind(&formatted_mobile)
        .fetch_optional(&self.pool)
        .await?;

        Ok(patient)
    }

    pub async fn search(&self, query: &str, organization_id: Uuid, limit: i64) -> Result<Vec<Patient>> {
        let patients = sqlx::query_as::<_, Patient>(
            r#"
            SELECT *
            FROM patient
            WHERE organization_id = $1
              AND is_deleted = false
              AND (
                mrn_number ILIKE $2
                OR mobile_number ILIKE $2
                OR to_tsvector('english', full_name) @@ plainto_tsquery('english', $3)
              )
            ORDER BY created_at DESC
            LIMIT $4
            "#,
        )
        .bind(organization_id)
        .bind(format!("%{}%", query))
        .bind(query)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(patients)
    }
}
