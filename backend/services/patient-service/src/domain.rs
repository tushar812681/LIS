use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use common::types::{Gender, Language, CommunicationChannel, RegistrationSource};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
#[graphql(rename_fields = "camelCase")]
pub struct Patient {
    pub id: Uuid,
    pub mrn_number: String,
    pub organization_id: Uuid,

    // Demographics
    pub salutation: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub age: i32,
    pub gender: Gender,
    pub blood_group: Option<String>,

    // Identity
    pub aadhaar_number: Option<String>, // Encrypted
    pub aadhaar_verified: bool,
    pub pan_number: Option<String>,
    pub passport_number: Option<String>,
    pub abdm_health_id: Option<String>,
    pub abdm_phr_address: Option<String>,

    // Contact
    pub mobile_number: String,
    pub alternate_mobile: Option<String>,
    pub email: Option<String>,
    pub preferred_language: Language,
    pub preferred_communication: CommunicationChannel,

    // Additional
    pub occupation: Option<String>,
    pub marital_status: Option<String>,
    pub nationality: String,
    pub profile_photo_url: Option<String>,

    // Metadata
    pub registration_source: RegistrationSource,
    pub registration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub is_active: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Patient {
    pub fn calculate_age(&mut self) {
        self.age = common::utils::calculate_age(self.date_of_birth);
    }

    pub fn build_full_name(&mut self) {
        let mut parts = Vec::new();

        if let Some(salutation) = &self.salutation {
            parts.push(salutation.as_str());
        }

        parts.push(&self.first_name);

        if let Some(middle_name) = &self.middle_name {
            parts.push(middle_name);
        }

        if let Some(last_name) = &self.last_name {
            parts.push(last_name);
        }

        self.full_name = parts.join(" ");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
#[graphql(rename_fields = "camelCase")]
pub struct PatientAddress {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub address_type: String, // HOME, WORK, TEMPORARY
    pub is_primary: bool,

    pub address_line1: String,
    pub address_line2: Option<String>,
    pub landmark: Option<String>,
    pub city: String,
    pub district: Option<String>,
    pub state: String,
    pub country: String,
    pub pincode: String,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
#[graphql(rename_fields = "camelCase")]
pub struct PatientConsent {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub consent_type: String, // DATA_PROCESSING, COMMUNICATION, MARKETING, DATA_SHARING, RESEARCH
    pub status: String,       // GRANTED, WITHDRAWN, EXPIRED
    pub granted_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub consent_text: String,
    pub consent_version: String,
    pub ip_address: String,
    pub device_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(rename_fields = "camelCase")]
pub struct CreatePatientInput {
    pub salutation: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: NaiveDate,
    pub gender: Gender,
    pub mobile_number: String,
    pub alternate_mobile: Option<String>,
    pub email: Option<String>,
    pub aadhaar_number: Option<String>,
    pub preferred_language: Option<Language>,
    pub preferred_communication: Option<CommunicationChannel>,
    pub occupation: Option<String>,
    pub marital_status: Option<String>,
    pub nationality: Option<String>,
}

impl CreatePatientInput {
    pub fn validate(&self) -> Result<(), common::error::Error> {
        // Validate required fields
        if self.first_name.trim().is_empty() {
            return Err(common::error::Error::InvalidInput(
                "First name is required".to_string(),
            ));
        }

        // Validate phone number
        if !common::utils::is_valid_indian_phone(&self.mobile_number) {
            return Err(common::error::Error::InvalidInput(
                "Invalid Indian mobile number".to_string(),
            ));
        }

        // Validate email if provided
        if let Some(email) = &self.email {
            if !email.contains('@') {
                return Err(common::error::Error::InvalidInput(
                    "Invalid email format".to_string(),
                ));
            }
        }

        // Validate Aadhaar if provided
        if let Some(aadhaar) = &self.aadhaar_number {
            if !common::utils::is_valid_aadhaar(aadhaar) {
                return Err(common::error::Error::InvalidInput(
                    "Invalid Aadhaar number".to_string(),
                ));
            }
        }

        // Validate date of birth (not in future, reasonable age)
        if self.date_of_birth > chrono::Utc::now().naive_utc().date() {
            return Err(common::error::Error::InvalidInput(
                "Date of birth cannot be in the future".to_string(),
            ));
        }

        let age = common::utils::calculate_age(self.date_of_birth);
        if age < 0 || age > 150 {
            return Err(common::error::Error::InvalidInput(
                "Invalid age calculated from date of birth".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(rename_fields = "camelCase")]
pub struct UpdatePatientInput {
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile_number: Option<String>,
    pub alternate_mobile: Option<String>,
    pub email: Option<String>,
    pub preferred_language: Option<Language>,
    pub preferred_communication: Option<CommunicationChannel>,
    pub occupation: Option<String>,
    pub marital_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "camelCase")]
pub struct PotentialDuplicate {
    pub patient_id: Uuid,
    pub confidence: f64,
    pub match_reason: String,
}
