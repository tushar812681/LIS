use sqlx::PgPool;
use uuid::Uuid;

#[cfg(test)]
mod patient_repository_tests {
    use super::*;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/lis_patient_test".to_string());

        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_create_patient() {
        let pool = setup_test_db().await;

        // Test patient creation
        let patient_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();

        let result = sqlx::query(
            "INSERT INTO patient (id, organization_id, first_name, last_name, date_of_birth, gender, mobile, email, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(patient_id)
        .bind(org_id)
        .bind("John")
        .bind("Doe")
        .bind(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap())
        .bind("MALE")
        .bind("9876543210")
        .bind("john.doe@example.com")
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Patient creation should succeed");

        // Cleanup
        sqlx::query("DELETE FROM patient WHERE id = $1")
            .bind(patient_id)
            .execute(&pool)
            .await
            .ok();
    }

    #[tokio::test]
    async fn test_find_patient_by_mrn() {
        let pool = setup_test_db().await;

        let patient_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let mrn = format!("MRN-TEST-{}", Uuid::new_v4());

        // Create test patient
        sqlx::query(
            "INSERT INTO patient (id, organization_id, mrn, first_name, last_name, date_of_birth, gender, mobile, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(patient_id)
        .bind(org_id)
        .bind(&mrn)
        .bind("Jane")
        .bind("Smith")
        .bind(chrono::NaiveDate::from_ymd_opt(1985, 5, 15).unwrap())
        .bind("FEMALE")
        .bind("9876543211")
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .expect("Failed to create test patient");

        // Find by MRN
        let result = sqlx::query("SELECT * FROM patient WHERE mrn = $1 AND is_deleted = FALSE")
            .bind(&mrn)
            .fetch_optional(&pool)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some(), "Should find patient by MRN");

        // Cleanup
        sqlx::query("DELETE FROM patient WHERE id = $1")
            .bind(patient_id)
            .execute(&pool)
            .await
            .ok();
    }

    #[tokio::test]
    async fn test_patient_duplicate_mobile_validation() {
        let pool = setup_test_db().await;

        let org_id = Uuid::new_v4();
        let mobile = "9999888877";

        // Create first patient
        let patient1_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO patient (id, organization_id, first_name, last_name, date_of_birth, gender, mobile, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(patient1_id)
        .bind(org_id)
        .bind("Patient")
        .bind("One")
        .bind(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap())
        .bind("MALE")
        .bind(mobile)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .expect("First patient should be created");

        // Try to create second patient with same mobile in same org
        let patient2_id = Uuid::new_v4();
        let result = sqlx::query(
            "INSERT INTO patient (id, organization_id, first_name, last_name, date_of_birth, gender, mobile, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(patient2_id)
        .bind(org_id)
        .bind("Patient")
        .bind("Two")
        .bind(chrono::NaiveDate::from_ymd_opt(1991, 1, 1).unwrap())
        .bind("FEMALE")
        .bind(mobile)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await;

        // Should fail due to unique constraint
        assert!(result.is_err(), "Duplicate mobile should be rejected");

        // Cleanup
        sqlx::query("DELETE FROM patient WHERE id = $1")
            .bind(patient1_id)
            .execute(&pool)
            .await
            .ok();
    }

    #[tokio::test]
    async fn test_soft_delete_patient() {
        let pool = setup_test_db().await;

        let patient_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();

        // Create patient
        sqlx::query(
            "INSERT INTO patient (id, organization_id, first_name, last_name, date_of_birth, gender, mobile, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(patient_id)
        .bind(org_id)
        .bind("Delete")
        .bind("Test")
        .bind(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap())
        .bind("MALE")
        .bind("9876543299")
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .expect("Patient creation failed");

        // Soft delete
        let deleted_by = Uuid::new_v4();
        sqlx::query(
            "UPDATE patient SET is_deleted = TRUE, deleted_at = NOW(), deleted_by = $1 WHERE id = $2"
        )
        .bind(deleted_by)
        .bind(patient_id)
        .execute(&pool)
        .await
        .expect("Soft delete failed");

        // Verify deleted flag
        let result: (bool,) = sqlx::query_as("SELECT is_deleted FROM patient WHERE id = $1")
            .bind(patient_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch patient");

        assert!(result.0, "Patient should be marked as deleted");

        // Cleanup
        sqlx::query("DELETE FROM patient WHERE id = $1")
            .bind(patient_id)
            .execute(&pool)
            .await
            .ok();
    }
}
