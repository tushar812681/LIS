use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "gender", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BloodGroup {
    APositive,
    ANegative,
    BPositive,
    BNegative,
    OPositive,
    ONegative,
    ABPositive,
    ABNegative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "priority", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
    Stat,    // Immediate (<1 hour)
    Urgent,  // Priority (<4 hours)
    Routine, // Normal TAT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "specimen_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SampleType {
    #[sqlx(rename = "BLOOD")]
    WholeBlood,
    Serum,
    Plasma,
    Urine,
    Stool,
    Sputum,
    #[sqlx(rename = "CSF")]
    Csf, // Cerebrospinal Fluid
    #[sqlx(rename = "TISSUE")]
    Tissue,
    #[sqlx(rename = "SWAB")]
    Swab,
    #[sqlx(rename = "BIOPSY")]
    Biopsy,
    #[sqlx(rename = "ASPIRATE")]
    Aspirate,
    #[sqlx(rename = "OTHER")]
    Other,
    SynovialFluid,
    PleuralFluid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "sample_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SampleStatus {
    Pending,     // Ordered, not yet collected
    Collected,   // Collected, in transit
    Received,    // Received at lab
    Processing,  // Being processed
    Available,   // Ready for testing
    InProgress,  // On analyzer
    Tested,      // All tests complete
    Rejected,    // Rejected sample
    Disposed,    // Disposed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "order_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    PendingPayment,
    Confirmed,
    SampleCollected,
    InProgress,
    PartiallyCompleted,
    Completed,
    Cancelled,
    OnHold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Unpaid,
    PartiallyPaid,
    Paid,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultFlag {
    Normal,
    High,
    Low,
    CriticalHigh,
    CriticalLow,
    Abnormal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    Pending,
    AutoVerified,
    TechnicianVerified,
    PathologistVerified,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquipmentStatus {
    Online,
    Offline,
    Busy,
    Error,
    Maintenance,
    Calibrating,
    RunningQc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "communication_channel", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommunicationChannel {
    #[sqlx(rename = "WHATSAPP")]
    WhatsApp,
    Sms,
    Email,
    Portal,
    PushNotification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "language", rename_all = "lowercase")]
pub enum Language {
    #[serde(rename = "en")]
    #[sqlx(rename = "en")]
    English,
    #[serde(rename = "hi")]
    #[sqlx(rename = "hi")]
    Hindi,
    #[serde(rename = "ta")]
    #[sqlx(rename = "ta")]
    Tamil,
    #[serde(rename = "te")]
    #[sqlx(rename = "te")]
    Telugu,
    #[serde(rename = "kn")]
    #[sqlx(rename = "kn")]
    Kannada,
    #[serde(rename = "bn")]
    #[sqlx(rename = "bn")]
    Bengali,
    #[serde(rename = "mr")]
    #[sqlx(rename = "mr")]
    Marathi,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "registration_source", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationSource {
    WalkIn,
    WebPortal,
    MobileApp,
    #[sqlx(rename = "WHATSAPP")]
    WhatsApp,
    Abdm,
    Import,
}

impl Default for RegistrationSource {
    fn default() -> Self {
        Self::WalkIn
    }
}
