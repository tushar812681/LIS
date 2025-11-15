use async_graphql::{Context, Object, Result, ErrorExtensions};
use uuid::Uuid;
use common::pagination::PaginationParams;
use crate::domain::*;
use crate::service::OrganizationService;

// ============================================================================
// Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get organization by ID
    async fn organization(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<Organization>> {
        let service = ctx.data::<OrganizationService>()?;
        let org = service.get_organization(id).await?;
        Ok(Some(org))
    }

    /// Get organization by code
    async fn organization_by_code(
        &self,
        ctx: &Context<'_>,
        org_code: String,
    ) -> Result<Option<Organization>> {
        let service = ctx.data::<OrganizationService>()?;
        let org = service.get_organization_by_code(org_code).await?;
        Ok(Some(org))
    }

    /// List organizations with filtering and pagination
    async fn organizations(
        &self,
        ctx: &Context<'_>,
        filter: Option<OrganizationFilter>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<OrganizationPaginated> {
        let service = ctx.data::<OrganizationService>()?;

        let pagination = PaginationParams {
            page: page.unwrap_or(1) as u32,
            page_size: page_size.unwrap_or(20) as u32,
        };

        let paginated = service.list_organizations(filter, pagination).await.map_err(|e| e.extend())?;

        Ok(OrganizationPaginated {
            data: paginated.edges.into_iter().map(|edge| edge.node).collect(),
            total: paginated.page_info.total_count as i32,
            page: paginated.page_info.current_page as i32,
            page_size: paginated.page_info.page_size as i32,
            total_pages: paginated.page_info.total_pages as i32,
        })
    }

    /// Get branch by ID
    async fn branch(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<OrganizationBranch>> {
        let service = ctx.data::<OrganizationService>()?;
        let branch = service.get_branch(id).await?;
        Ok(Some(branch))
    }

    /// List branches for an organization
    async fn branches(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
        filter: Option<BranchFilter>,
    ) -> Result<Vec<OrganizationBranch>> {
        let service = ctx.data::<OrganizationService>()?;
        Ok(service.list_branches(organization_id, filter).await.map_err(|e| e.extend())?)
    }

    /// Get accreditation by ID
    async fn accreditation(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<Accreditation>> {
        let service = ctx.data::<OrganizationService>()?;
        let accreditation = service.get_accreditation(id).await?;
        Ok(Some(accreditation))
    }

    /// List accreditations for an organization
    async fn accreditations(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
    ) -> Result<Vec<Accreditation>> {
        let service = ctx.data::<OrganizationService>()?;
        Ok(service.list_accreditations(organization_id).await.map_err(|e| e.extend())?)
    }

    /// Get department by ID
    async fn department(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<Department>> {
        let service = ctx.data::<OrganizationService>()?;
        let department = service.get_department(id).await?;
        Ok(Some(department))
    }

    /// List departments for an organization
    async fn departments(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
        filter: Option<DepartmentFilter>,
    ) -> Result<Vec<Department>> {
        let service = ctx.data::<OrganizationService>()?;
        Ok(service.list_departments(organization_id, filter).await.map_err(|e| e.extend())?)
    }

    /// Get organization setting
    async fn organization_setting(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
        category: String,
        key: String,
    ) -> Result<Option<OrganizationSetting>> {
        let service = ctx.data::<OrganizationService>()?;
        let setting = service.get_setting(organization_id, category, key).await?;
        Ok(Some(setting))
    }

    /// List organization settings
    async fn organization_settings(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
        category: Option<String>,
    ) -> Result<Vec<OrganizationSetting>> {
        let service = ctx.data::<OrganizationService>()?;
        Ok(service.list_settings(organization_id, category).await.map_err(|e| e.extend())?)
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new organization
    async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<Organization> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_organization(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update organization
    async fn update_organization(
        &self,
        ctx: &Context<'_>,
        input: UpdateOrganizationInput,
    ) -> Result<Organization> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_organization(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Update organization status
    async fn update_organization_status(
        &self,
        ctx: &Context<'_>,
        input: UpdateOrganizationStatusInput,
    ) -> Result<Organization> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_organization_status(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Update organization subscription
    async fn update_subscription(
        &self,
        ctx: &Context<'_>,
        input: UpdateSubscriptionInput,
    ) -> Result<Organization> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_subscription(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Delete organization (soft delete)
    async fn delete_organization(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get deleted_by from authenticated user context
        let deleted_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.delete_organization(id, deleted_by).await.map_err(|e| e.extend())?)
    }

    /// Create a new branch
    async fn create_branch(
        &self,
        ctx: &Context<'_>,
        input: CreateBranchInput,
    ) -> Result<OrganizationBranch> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_branch(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update branch
    async fn update_branch(
        &self,
        ctx: &Context<'_>,
        input: UpdateBranchInput,
    ) -> Result<OrganizationBranch> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_branch(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Deactivate branch
    async fn deactivate_branch(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.deactivate_branch(id, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Add accreditation
    async fn add_accreditation(
        &self,
        ctx: &Context<'_>,
        input: AddAccreditationInput,
    ) -> Result<Accreditation> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.add_accreditation(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update accreditation
    async fn update_accreditation(
        &self,
        ctx: &Context<'_>,
        input: UpdateAccreditationInput,
    ) -> Result<Accreditation> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_accreditation(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Deactivate accreditation
    async fn deactivate_accreditation(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.deactivate_accreditation(id, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Create department
    async fn create_department(
        &self,
        ctx: &Context<'_>,
        input: CreateDepartmentInput,
    ) -> Result<Department> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_department(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update department
    async fn update_department(
        &self,
        ctx: &Context<'_>,
        input: UpdateDepartmentInput,
    ) -> Result<Department> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_department(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Deactivate department
    async fn deactivate_department(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.deactivate_department(id, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Update organization setting (upsert)
    async fn update_organization_setting(
        &self,
        ctx: &Context<'_>,
        input: UpdateOrganizationSettingInput,
    ) -> Result<OrganizationSetting> {
        let service = ctx.data::<OrganizationService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_setting(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Delete organization setting
    async fn delete_organization_setting(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
        category: String,
        key: String,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;
        Ok(service.delete_setting(organization_id, category, key).await.map_err(|e| e.extend())?)
    }

    /// Increment test counter (called by other services when processing tests)
    async fn increment_test_counter(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<OrganizationService>()?;
        service.increment_test_counter(organization_id).await?;
        Ok(true)
    }
}

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(async_graphql::SimpleObject)]
pub struct OrganizationPaginated {
    pub data: Vec<Organization>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}
