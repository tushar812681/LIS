use sqlx::PgPool;
use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;

// ============================================================================
// Organization Repository
// ============================================================================

#[derive(Clone)]
pub struct OrganizationRepository {
    pool: PgPool,
}

impl OrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateOrganizationInput, created_by: Uuid) -> Result<Organization> {
        let id = Uuid::new_v4();

        // Generate organization code
        let org_code: (String,) = sqlx::query_as("SELECT generate_org_code()")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let subscription_start_date = chrono::Local::now().date_naive();
        let subscription_end_date = subscription_start_date + chrono::Duration::days(30); // 30-day trial

        let organization = sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organization (
                id, org_code, organization_name, legal_name, short_name, organization_type,
                organization_status, email, phone, website,
                address_line1, address_line2, city, state, country, postal_code,
                contact_person_name, contact_person_email, contact_person_phone,
                subscription_plan, subscription_start_date, subscription_end_date,
                max_users, max_branches, max_tests_per_month,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(org_code.0)
        .bind(&input.organization_name)
        .bind(&input.legal_name)
        .bind(&input.short_name)
        .bind(&input.organization_type)
        .bind(OrganizationStatus::Trial)
        .bind(&input.email)
        .bind(&input.phone)
        .bind(&input.website)
        .bind(&input.address_line1)
        .bind(&input.address_line2)
        .bind(&input.city)
        .bind(&input.state)
        .bind(input.country.as_ref().unwrap_or(&"India".to_string()))
        .bind(&input.postal_code)
        .bind(&input.contact_person_name)
        .bind(&input.contact_person_email)
        .bind(&input.contact_person_phone)
        .bind(input.subscription_plan.unwrap_or(SubscriptionPlan::Free))
        .bind(subscription_start_date)
        .bind(subscription_end_date)
        .bind(5) // Default max users
        .bind(1) // Default max branches
        .bind(1000) // Default max tests per month
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Organization>> {
        let organization = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organization WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn find_by_org_code(&self, org_code: &str) -> Result<Option<Organization>> {
        let organization = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organization WHERE org_code = $1 AND is_deleted = FALSE"
        )
        .bind(org_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<Organization>> {
        let organization = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organization WHERE email = $1 AND is_deleted = FALSE"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn list(
        &self,
        filter: Option<OrganizationFilter>,
        pagination: PaginationParams,
    ) -> Result<Paginated<Organization>> {
        let mut query = String::from(
            "FROM organization WHERE is_deleted = FALSE"
        );
        let mut count_query = format!("SELECT COUNT(*) {}", query);
        let mut bindings = vec![];

        if let Some(f) = filter {
            if let Some(status) = f.organization_status {
                bindings.push(format!("organization_status = '{:?}'", status));
            }
            if let Some(org_type) = f.organization_type {
                bindings.push(format!("organization_type = '{:?}'", org_type));
            }
            if let Some(plan) = f.subscription_plan {
                bindings.push(format!("subscription_plan = '{:?}'", plan));
            }
            if let Some(search) = f.search_query {
                bindings.push(format!(
                    "(organization_name ILIKE '%{}%' OR email ILIKE '%{}%')",
                    search, search
                ));
            }
        }

        if !bindings.is_empty() {
            let where_clause = format!(" AND {}", bindings.join(" AND "));
            query.push_str(&where_clause);
            count_query.push_str(&where_clause);
        }

        // Get total count
        let total: (i64,) = sqlx::query_as(&count_query)
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        // Get paginated results
        let select_query = format!(
            "SELECT * {} ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            query
        );

        let organizations = sqlx::query_as::<_, Organization>(&select_query)
            .bind(pagination.page_size as i64)
            .bind(((pagination.page - 1) * pagination.page_size) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(Paginated::new(
            organizations,
            &pagination,
            total.0 as u64,
        ))
    }

    pub async fn update(&self, input: UpdateOrganizationInput, updated_by: Uuid) -> Result<Organization> {
        let mut updates = vec!["updated_by = $2".to_string()];
        let mut bind_count = 3;

        if input.organization_name.is_some() {
            updates.push(format!("organization_name = ${}", bind_count));
            bind_count += 1;
        }
        if input.legal_name.is_some() {
            updates.push(format!("legal_name = ${}", bind_count));
            bind_count += 1;
        }
        if input.email.is_some() {
            updates.push(format!("email = ${}", bind_count));
            bind_count += 1;
        }

        let update_query = format!(
            "UPDATE organization SET {} WHERE id = $1 AND is_deleted = FALSE RETURNING *",
            updates.join(", ")
        );

        let mut query = sqlx::query_as::<_, Organization>(&update_query)
            .bind(input.id)
            .bind(updated_by);

        if let Some(name) = input.organization_name {
            query = query.bind(name);
        }
        if let Some(legal_name) = input.legal_name {
            query = query.bind(legal_name);
        }
        if let Some(email) = input.email {
            query = query.bind(email);
        }

        let organization = query
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn update_status(&self, input: UpdateOrganizationStatusInput, updated_by: Uuid) -> Result<Organization> {
        let organization = sqlx::query_as::<_, Organization>(
            r#"
            UPDATE organization
            SET organization_status = $2, updated_by = $3
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.status)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn update_subscription(&self, input: UpdateSubscriptionInput, updated_by: Uuid) -> Result<Organization> {
        let organization = sqlx::query_as::<_, Organization>(
            r#"
            UPDATE organization
            SET subscription_plan = $2,
                subscription_start_date = $3,
                subscription_end_date = $4,
                max_users = $5,
                max_branches = $6,
                max_tests_per_month = $7,
                updated_by = $8
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.subscription_plan)
        .bind(input.subscription_start_date)
        .bind(input.subscription_end_date)
        .bind(input.max_users)
        .bind(input.max_branches)
        .bind(input.max_tests_per_month)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(organization)
    }

    pub async fn delete(&self, id: Uuid, deleted_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE organization SET is_deleted = TRUE, deleted_at = NOW(), deleted_by = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(deleted_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn increment_test_counter(&self, organization_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE organization SET current_month_tests = current_month_tests + 1 WHERE id = $1"
        )
        .bind(organization_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }
}

// ============================================================================
// Organization Branch Repository
// ============================================================================

#[derive(Clone)]
pub struct OrganizationBranchRepository {
    pool: PgPool,
}

impl OrganizationBranchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateBranchInput, created_by: Uuid) -> Result<OrganizationBranch> {
        let id = Uuid::new_v4();

        let branch = sqlx::query_as::<_, OrganizationBranch>(
            r#"
            INSERT INTO organization_branch (
                id, organization_id, branch_code, branch_name,
                is_active, is_main_branch,
                email, phone,
                address_line1, address_line2, city, state, country, postal_code,
                manager_id, manager_name, manager_email, manager_phone,
                sample_processing_capacity,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.branch_code)
        .bind(&input.branch_name)
        .bind(true)
        .bind(input.is_main_branch.unwrap_or(false))
        .bind(&input.email)
        .bind(&input.phone)
        .bind(&input.address_line1)
        .bind(&input.address_line2)
        .bind(&input.city)
        .bind(&input.state)
        .bind(input.country.as_ref().unwrap_or(&"India".to_string()))
        .bind(&input.postal_code)
        .bind(input.manager_id)
        .bind(&input.manager_name)
        .bind(&input.manager_email)
        .bind(&input.manager_phone)
        .bind(input.sample_processing_capacity)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(branch)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<OrganizationBranch>> {
        let branch = sqlx::query_as::<_, OrganizationBranch>(
            "SELECT * FROM organization_branch WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(branch)
    }

    pub async fn list_by_organization(
        &self,
        organization_id: Uuid,
        filter: Option<BranchFilter>,
    ) -> Result<Vec<OrganizationBranch>> {
        let mut query = String::from(
            "SELECT * FROM organization_branch WHERE organization_id = $1"
        );
        let mut conditions = vec![];

        if let Some(f) = filter {
            if let Some(is_active) = f.is_active {
                conditions.push(format!("is_active = {}", is_active));
            }
            if let Some(city) = f.city {
                conditions.push(format!("city = '{}'", city));
            }
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY is_main_branch DESC, branch_name ASC");

        let branches = sqlx::query_as::<_, OrganizationBranch>(&query)
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(branches)
    }

    pub async fn update(&self, input: UpdateBranchInput, updated_by: Uuid) -> Result<OrganizationBranch> {
        let branch = sqlx::query_as::<_, OrganizationBranch>(
            r#"
            UPDATE organization_branch
            SET branch_name = COALESCE($2, branch_name),
                email = COALESCE($3, email),
                phone = COALESCE($4, phone),
                manager_name = COALESCE($5, manager_name),
                sample_processing_capacity = COALESCE($6, sample_processing_capacity),
                updated_by = $7
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.branch_name)
        .bind(input.email)
        .bind(input.phone)
        .bind(input.manager_name)
        .bind(input.sample_processing_capacity)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(branch)
    }

    pub async fn deactivate(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE organization_branch SET is_active = FALSE, updated_by = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(updated_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_by_organization(&self, organization_id: Uuid) -> Result<i32> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM organization_branch WHERE organization_id = $1 AND is_active = TRUE"
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(count.0 as i32)
    }
}

// ============================================================================
// Accreditation Repository
// ============================================================================

#[derive(Clone)]
pub struct AccreditationRepository {
    pool: PgPool,
}

impl AccreditationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: AddAccreditationInput, created_by: Uuid) -> Result<Accreditation> {
        let id = Uuid::new_v4();

        let accreditation = sqlx::query_as::<_, Accreditation>(
            r#"
            INSERT INTO accreditation (
                id, organization_id, accreditation_type, accreditation_number,
                issuing_authority, issue_date, expiry_date,
                scope_of_accreditation, certificate_url,
                is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(input.accreditation_type)
        .bind(&input.accreditation_number)
        .bind(&input.issuing_authority)
        .bind(input.issue_date)
        .bind(input.expiry_date)
        .bind(&input.scope_of_accreditation)
        .bind(&input.certificate_url)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(accreditation)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Accreditation>> {
        let accreditation = sqlx::query_as::<_, Accreditation>(
            "SELECT * FROM accreditation WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(accreditation)
    }

    pub async fn list_by_organization(&self, organization_id: Uuid) -> Result<Vec<Accreditation>> {
        let accreditations = sqlx::query_as::<_, Accreditation>(
            "SELECT * FROM accreditation WHERE organization_id = $1 AND is_active = TRUE ORDER BY expiry_date DESC"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(accreditations)
    }

    pub async fn update(&self, input: UpdateAccreditationInput, updated_by: Uuid) -> Result<Accreditation> {
        let accreditation = sqlx::query_as::<_, Accreditation>(
            r#"
            UPDATE accreditation
            SET expiry_date = COALESCE($2, expiry_date),
                scope_of_accreditation = COALESCE($3, scope_of_accreditation),
                certificate_url = COALESCE($4, certificate_url),
                updated_by = $5
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.expiry_date)
        .bind(input.scope_of_accreditation)
        .bind(input.certificate_url)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(accreditation)
    }

    pub async fn deactivate(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE accreditation SET is_active = FALSE, updated_by = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(updated_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// Department Repository
// ============================================================================

#[derive(Clone)]
pub struct DepartmentRepository {
    pool: PgPool,
}

impl DepartmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateDepartmentInput, created_by: Uuid) -> Result<Department> {
        let id = Uuid::new_v4();

        let department = sqlx::query_as::<_, Department>(
            r#"
            INSERT INTO department (
                id, organization_id, branch_id, department_code, department_name,
                description, hod_user_id, hod_name, email, phone,
                is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(input.branch_id)
        .bind(&input.department_code)
        .bind(&input.department_name)
        .bind(&input.description)
        .bind(input.hod_user_id)
        .bind(&input.hod_name)
        .bind(&input.email)
        .bind(&input.phone)
        .bind(true)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(department)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Department>> {
        let department = sqlx::query_as::<_, Department>(
            "SELECT * FROM department WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(department)
    }

    pub async fn list_by_organization(
        &self,
        organization_id: Uuid,
        filter: Option<DepartmentFilter>,
    ) -> Result<Vec<Department>> {
        let mut query = String::from(
            "SELECT * FROM department WHERE organization_id = $1"
        );
        let mut conditions = vec![];

        if let Some(f) = filter {
            if let Some(branch_id) = f.branch_id {
                conditions.push(format!("branch_id = '{}'", branch_id));
            }
            if let Some(is_active) = f.is_active {
                conditions.push(format!("is_active = {}", is_active));
            }
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }

        query.push_str(" ORDER BY department_name ASC");

        let departments = sqlx::query_as::<_, Department>(&query)
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(departments)
    }

    pub async fn update(&self, input: UpdateDepartmentInput, updated_by: Uuid) -> Result<Department> {
        let department = sqlx::query_as::<_, Department>(
            r#"
            UPDATE department
            SET department_name = COALESCE($2, department_name),
                description = COALESCE($3, description),
                hod_user_id = COALESCE($4, hod_user_id),
                hod_name = COALESCE($5, hod_name),
                email = COALESCE($6, email),
                phone = COALESCE($7, phone),
                updated_by = $8
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(input.id)
        .bind(input.department_name)
        .bind(input.description)
        .bind(input.hod_user_id)
        .bind(input.hod_name)
        .bind(input.email)
        .bind(input.phone)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(department)
    }

    pub async fn deactivate(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE department SET is_active = FALSE, updated_by = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(updated_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================================================
// Organization Setting Repository
// ============================================================================

#[derive(Clone)]
pub struct OrganizationSettingRepository {
    pool: PgPool,
}

impl OrganizationSettingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, input: UpdateOrganizationSettingInput, updated_by: Uuid) -> Result<OrganizationSetting> {
        let id = Uuid::new_v4();

        let setting = sqlx::query_as::<_, OrganizationSetting>(
            r#"
            INSERT INTO organization_setting (
                id, organization_id, setting_category, setting_key, setting_value,
                setting_type, description, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (organization_id, setting_category, setting_key)
            DO UPDATE SET
                setting_value = EXCLUDED.setting_value,
                setting_type = EXCLUDED.setting_type,
                description = EXCLUDED.description,
                updated_by = EXCLUDED.updated_by,
                updated_at = NOW()
            RETURNING *
            "#
        )
        .bind(id)
        .bind(input.organization_id)
        .bind(&input.setting_category)
        .bind(&input.setting_key)
        .bind(&input.setting_value)
        .bind(input.setting_type)
        .bind(input.description)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(setting)
    }

    pub async fn get(
        &self,
        organization_id: Uuid,
        category: &str,
        key: &str,
    ) -> Result<Option<OrganizationSetting>> {
        let setting = sqlx::query_as::<_, OrganizationSetting>(
            "SELECT * FROM organization_setting WHERE organization_id = $1 AND setting_category = $2 AND setting_key = $3"
        )
        .bind(organization_id)
        .bind(category)
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(setting)
    }

    pub async fn list_by_organization(
        &self,
        organization_id: Uuid,
        category: Option<String>,
    ) -> Result<Vec<OrganizationSetting>> {
        let query = if let Some(cat) = category {
            sqlx::query_as::<_, OrganizationSetting>(
                "SELECT * FROM organization_setting WHERE organization_id = $1 AND setting_category = $2 ORDER BY setting_key"
            )
            .bind(organization_id)
            .bind(cat)
        } else {
            sqlx::query_as::<_, OrganizationSetting>(
                "SELECT * FROM organization_setting WHERE organization_id = $1 ORDER BY setting_category, setting_key"
            )
            .bind(organization_id)
        };

        let settings = query
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(settings)
    }

    pub async fn delete(
        &self,
        organization_id: Uuid,
        category: &str,
        key: &str,
    ) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM organization_setting WHERE organization_id = $1 AND setting_category = $2 AND setting_key = $3"
        )
        .bind(organization_id)
        .bind(category)
        .bind(key)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected() > 0)
    }
}
