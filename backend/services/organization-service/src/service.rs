use uuid::Uuid;
use common::error::{Error, Result};
use common::pagination::{Paginated, PaginationParams};
use crate::domain::*;
use crate::repository::*;

// ============================================================================
// Organization Service
// ============================================================================

#[derive(Clone)]
pub struct OrganizationService {
    org_repo: OrganizationRepository,
    branch_repo: OrganizationBranchRepository,
    accreditation_repo: AccreditationRepository,
    department_repo: DepartmentRepository,
    setting_repo: OrganizationSettingRepository,
}

impl OrganizationService {
    pub fn new(
        org_repo: OrganizationRepository,
        branch_repo: OrganizationBranchRepository,
        accreditation_repo: AccreditationRepository,
        department_repo: DepartmentRepository,
        setting_repo: OrganizationSettingRepository,
    ) -> Self {
        Self {
            org_repo,
            branch_repo,
            accreditation_repo,
            department_repo,
            setting_repo,
        }
    }

    // ========================================================================
    // Organization Operations
    // ========================================================================

    pub async fn create_organization(
        &self,
        input: CreateOrganizationInput,
        created_by: Uuid,
    ) -> Result<Organization> {
        // Validate email uniqueness
        if let Some(_) = self.org_repo.find_by_email(&input.email).await? {
            return Err(Error::Validation(
                "Organization with this email already exists".to_string(),
            ));
        }

        // Validate organization name
        if input.organization_name.trim().is_empty() {
            return Err(Error::Validation(
                "Organization name cannot be empty".to_string(),
            ));
        }

        if input.organization_name.len() > 300 {
            return Err(Error::Validation(
                "Organization name too long (max 300 characters)".to_string(),
            ));
        }

        // Validate email format
        if !input.email.contains('@') || !input.email.contains('.') {
            return Err(Error::Validation("Invalid email format".to_string()));
        }

        let organization = self.org_repo.create(input, created_by).await?;

        tracing::info!(
            "Organization created: {} ({})",
            organization.organization_name,
            organization.org_code
        );

        Ok(organization)
    }

    pub async fn get_organization(&self, id: Uuid) -> Result<Organization> {
        self.org_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Organization not found".to_string()))
    }

    pub async fn get_organization_by_code(&self, org_code: String) -> Result<Organization> {
        self.org_repo
            .find_by_org_code(&org_code)
            .await?
            .ok_or_else(|| Error::NotFound("Organization not found".to_string()))
    }

    pub async fn list_organizations(
        &self,
        filter: Option<OrganizationFilter>,
        pagination: PaginationParams,
    ) -> Result<Paginated<Organization>> {
        self.org_repo.list(filter, pagination).await
    }

    pub async fn update_organization(
        &self,
        input: UpdateOrganizationInput,
        updated_by: Uuid,
    ) -> Result<Organization> {
        // Check if organization exists
        let existing = self.get_organization(input.id).await?;

        // Validate email uniqueness if changed
        if let Some(ref email) = input.email {
            if email != &existing.email {
                if let Some(_) = self.org_repo.find_by_email(email).await? {
                    return Err(Error::Validation(
                        "Another organization with this email already exists".to_string(),
                    ));
                }
            }
        }

        let organization = self.org_repo.update(input, updated_by).await?;

        tracing::info!(
            "Organization updated: {} ({})",
            organization.organization_name,
            organization.org_code
        );

        Ok(organization)
    }

    pub async fn update_organization_status(
        &self,
        input: UpdateOrganizationStatusInput,
        updated_by: Uuid,
    ) -> Result<Organization> {
        // Check if organization exists
        let existing = self.get_organization(input.id).await?;

        // Validate status transition
        if existing.organization_status == OrganizationStatus::Suspended
            && input.status == OrganizationStatus::Trial
        {
            return Err(Error::Validation(
                "Cannot change suspended organization to trial status".to_string(),
            ));
        }

        let organization = self.org_repo.update_status(input, updated_by).await?;

        tracing::info!(
            "Organization status updated: {} -> {:?}",
            organization.org_code,
            organization.organization_status
        );

        Ok(organization)
    }

    pub async fn update_subscription(
        &self,
        input: UpdateSubscriptionInput,
        updated_by: Uuid,
    ) -> Result<Organization> {
        // Check if organization exists
        let _ = self.get_organization(input.id).await?;

        // Validate dates
        if input.subscription_end_date <= input.subscription_start_date {
            return Err(Error::Validation(
                "Subscription end date must be after start date".to_string(),
            ));
        }

        // Validate limits
        if let Some(max_users) = input.max_users {
            if max_users <= 0 {
                return Err(Error::Validation(
                    "Max users must be greater than 0".to_string(),
                ));
            }
        }

        if let Some(max_branches) = input.max_branches {
            if max_branches <= 0 {
                return Err(Error::Validation(
                    "Max branches must be greater than 0".to_string(),
                ));
            }
        }

        let organization = self.org_repo.update_subscription(input, updated_by).await?;

        tracing::info!(
            "Subscription updated: {} -> {:?}",
            organization.org_code,
            organization.subscription_plan
        );

        Ok(organization)
    }

    pub async fn delete_organization(&self, id: Uuid, deleted_by: Uuid) -> Result<bool> {
        // Check if organization exists
        let organization = self.get_organization(id).await?;

        // Check if organization has active branches
        let branch_count = self.branch_repo.count_by_organization(id).await?;
        if branch_count > 0 {
            return Err(Error::Validation(
                "Cannot delete organization with active branches".to_string(),
            ));
        }

        let deleted = self.org_repo.delete(id, deleted_by).await?;

        if deleted {
            tracing::info!(
                "Organization deleted: {} ({})",
                organization.organization_name,
                organization.org_code
            );
        }

        Ok(deleted)
    }

    pub async fn increment_test_counter(&self, organization_id: Uuid) -> Result<()> {
        let organization = self.get_organization(organization_id).await?;

        if !organization.can_process_test() {
            return Err(Error::Validation(
                "Monthly test limit reached for this organization".to_string(),
            ));
        }

        self.org_repo.increment_test_counter(organization_id).await
    }

    // ========================================================================
    // Branch Operations
    // ========================================================================

    pub async fn create_branch(
        &self,
        input: CreateBranchInput,
        created_by: Uuid,
    ) -> Result<OrganizationBranch> {
        // Check if organization exists
        let organization = self.get_organization(input.organization_id).await?;

        // Check if organization can add more branches
        let current_branch_count = self
            .branch_repo
            .count_by_organization(input.organization_id)
            .await?;

        if !organization.can_add_branch(current_branch_count) {
            return Err(Error::Validation(format!(
                "Branch limit reached. Current plan allows {} branches",
                organization.max_branches.unwrap_or(1)
            )));
        }

        // Validate branch code
        if input.branch_code.trim().is_empty() {
            return Err(Error::Validation("Branch code cannot be empty".to_string()));
        }

        // Validate branch name
        if input.branch_name.trim().is_empty() {
            return Err(Error::Validation("Branch name cannot be empty".to_string()));
        }

        let branch = self.branch_repo.create(input, created_by).await?;

        tracing::info!(
            "Branch created: {} ({}) for organization {}",
            branch.branch_name,
            branch.branch_code,
            organization.org_code
        );

        Ok(branch)
    }

    pub async fn get_branch(&self, id: Uuid) -> Result<OrganizationBranch> {
        self.branch_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Branch not found".to_string()))
    }

    pub async fn list_branches(
        &self,
        organization_id: Uuid,
        filter: Option<BranchFilter>,
    ) -> Result<Vec<OrganizationBranch>> {
        // Verify organization exists
        let _ = self.get_organization(organization_id).await?;

        self.branch_repo
            .list_by_organization(organization_id, filter)
            .await
    }

    pub async fn update_branch(
        &self,
        input: UpdateBranchInput,
        updated_by: Uuid,
    ) -> Result<OrganizationBranch> {
        // Check if branch exists
        let _ = self.get_branch(input.id).await?;

        let branch = self.branch_repo.update(input, updated_by).await?;

        tracing::info!("Branch updated: {} ({})", branch.branch_name, branch.branch_code);

        Ok(branch)
    }

    pub async fn deactivate_branch(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        // Check if branch exists
        let branch = self.get_branch(id).await?;

        // Cannot deactivate main branch
        if branch.is_main() {
            return Err(Error::Validation(
                "Cannot deactivate main branch".to_string(),
            ));
        }

        let deactivated = self.branch_repo.deactivate(id, updated_by).await?;

        if deactivated {
            tracing::info!("Branch deactivated: {} ({})", branch.branch_name, branch.branch_code);
        }

        Ok(deactivated)
    }

    // ========================================================================
    // Accreditation Operations
    // ========================================================================

    pub async fn add_accreditation(
        &self,
        input: AddAccreditationInput,
        created_by: Uuid,
    ) -> Result<Accreditation> {
        // Check if organization exists
        let organization = self.get_organization(input.organization_id).await?;

        // Validate dates
        if input.expiry_date <= input.issue_date {
            return Err(Error::Validation(
                "Expiry date must be after issue date".to_string(),
            ));
        }

        // Validate accreditation number
        if input.accreditation_number.trim().is_empty() {
            return Err(Error::Validation(
                "Accreditation number cannot be empty".to_string(),
            ));
        }

        let accreditation = self.accreditation_repo.create(input, created_by).await?;

        tracing::info!(
            "Accreditation added: {:?} for organization {}",
            accreditation.accreditation_type,
            organization.org_code
        );

        Ok(accreditation)
    }

    pub async fn get_accreditation(&self, id: Uuid) -> Result<Accreditation> {
        self.accreditation_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Accreditation not found".to_string()))
    }

    pub async fn list_accreditations(&self, organization_id: Uuid) -> Result<Vec<Accreditation>> {
        // Verify organization exists
        let _ = self.get_organization(organization_id).await?;

        self.accreditation_repo
            .list_by_organization(organization_id)
            .await
    }

    pub async fn update_accreditation(
        &self,
        input: UpdateAccreditationInput,
        updated_by: Uuid,
    ) -> Result<Accreditation> {
        // Check if accreditation exists
        let existing = self.get_accreditation(input.id).await?;

        // Validate expiry date if provided
        if let Some(expiry_date) = input.expiry_date {
            if expiry_date <= existing.issue_date {
                return Err(Error::Validation(
                    "Expiry date must be after issue date".to_string(),
                ));
            }
        }

        let accreditation = self.accreditation_repo.update(input, updated_by).await?;

        tracing::info!(
            "Accreditation updated: {:?} ({})",
            accreditation.accreditation_type,
            accreditation.accreditation_number
        );

        Ok(accreditation)
    }

    pub async fn deactivate_accreditation(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        // Check if accreditation exists
        let accreditation = self.get_accreditation(id).await?;

        let deactivated = self.accreditation_repo.deactivate(id, updated_by).await?;

        if deactivated {
            tracing::info!(
                "Accreditation deactivated: {:?} ({})",
                accreditation.accreditation_type,
                accreditation.accreditation_number
            );
        }

        Ok(deactivated)
    }

    // ========================================================================
    // Department Operations
    // ========================================================================

    pub async fn create_department(
        &self,
        input: CreateDepartmentInput,
        created_by: Uuid,
    ) -> Result<Department> {
        // Check if organization exists
        let organization = self.get_organization(input.organization_id).await?;

        // Validate department code
        if input.department_code.trim().is_empty() {
            return Err(Error::Validation(
                "Department code cannot be empty".to_string(),
            ));
        }

        // Validate department name
        if input.department_name.trim().is_empty() {
            return Err(Error::Validation(
                "Department name cannot be empty".to_string(),
            ));
        }

        // If branch_id is provided, verify it exists and belongs to the organization
        if let Some(branch_id) = input.branch_id {
            let branch = self.get_branch(branch_id).await?;
            if branch.organization_id != input.organization_id {
                return Err(Error::Validation(
                    "Branch does not belong to this organization".to_string(),
                ));
            }
        }

        let department = self.department_repo.create(input, created_by).await?;

        tracing::info!(
            "Department created: {} ({}) for organization {}",
            department.department_name,
            department.department_code,
            organization.org_code
        );

        Ok(department)
    }

    pub async fn get_department(&self, id: Uuid) -> Result<Department> {
        self.department_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Department not found".to_string()))
    }

    pub async fn list_departments(
        &self,
        organization_id: Uuid,
        filter: Option<DepartmentFilter>,
    ) -> Result<Vec<Department>> {
        // Verify organization exists
        let _ = self.get_organization(organization_id).await?;

        self.department_repo
            .list_by_organization(organization_id, filter)
            .await
    }

    pub async fn update_department(
        &self,
        input: UpdateDepartmentInput,
        updated_by: Uuid,
    ) -> Result<Department> {
        // Check if department exists
        let _ = self.get_department(input.id).await?;

        let department = self.department_repo.update(input, updated_by).await?;

        tracing::info!(
            "Department updated: {} ({})",
            department.department_name,
            department.department_code
        );

        Ok(department)
    }

    pub async fn deactivate_department(&self, id: Uuid, updated_by: Uuid) -> Result<bool> {
        // Check if department exists
        let department = self.get_department(id).await?;

        let deactivated = self.department_repo.deactivate(id, updated_by).await?;

        if deactivated {
            tracing::info!(
                "Department deactivated: {} ({})",
                department.department_name,
                department.department_code
            );
        }

        Ok(deactivated)
    }

    // ========================================================================
    // Settings Operations
    // ========================================================================

    pub async fn update_setting(
        &self,
        input: UpdateOrganizationSettingInput,
        updated_by: Uuid,
    ) -> Result<OrganizationSetting> {
        // Check if organization exists
        let _ = self.get_organization(input.organization_id).await?;

        // Validate setting category
        if input.setting_category.trim().is_empty() {
            return Err(Error::Validation(
                "Setting category cannot be empty".to_string(),
            ));
        }

        // Validate setting key
        if input.setting_key.trim().is_empty() {
            return Err(Error::Validation("Setting key cannot be empty".to_string()));
        }

        let setting = self.setting_repo.upsert(input, updated_by).await?;

        tracing::info!(
            "Organization setting updated: {}.{}",
            setting.setting_category,
            setting.setting_key
        );

        Ok(setting)
    }

    pub async fn get_setting(
        &self,
        organization_id: Uuid,
        category: String,
        key: String,
    ) -> Result<OrganizationSetting> {
        // Check if organization exists
        let _ = self.get_organization(organization_id).await?;

        self.setting_repo
            .get(organization_id, &category, &key)
            .await?
            .ok_or_else(|| Error::NotFound("Setting not found".to_string()))
    }

    pub async fn list_settings(
        &self,
        organization_id: Uuid,
        category: Option<String>,
    ) -> Result<Vec<OrganizationSetting>> {
        // Verify organization exists
        let _ = self.get_organization(organization_id).await?;

        self.setting_repo
            .list_by_organization(organization_id, category)
            .await
    }

    pub async fn delete_setting(
        &self,
        organization_id: Uuid,
        category: String,
        key: String,
    ) -> Result<bool> {
        // Check if organization exists
        let _ = self.get_organization(organization_id).await?;

        let deleted = self
            .setting_repo
            .delete(organization_id, &category, &key)
            .await?;

        if deleted {
            tracing::info!("Organization setting deleted: {}.{}", category, key);
        }

        Ok(deleted)
    }
}
