use async_graphql::{Context, Object, Result, ErrorExtensions};
use uuid::Uuid;
use common::pagination::PaginationParams;
use crate::domain::*;
use crate::service::QcService;

// ============================================================================
// Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get QC material by ID
    async fn qc_material(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcMaterial>> {
        let service = ctx.data::<QcService>()?;
        let material = service.get_qc_material(id).await?;
        Ok(Some(material))
    }

    /// Get QC material by code
    async fn qc_material_by_code(
        &self,
        ctx: &Context<'_>,
        material_code: String,
    ) -> Result<Option<QcMaterial>> {
        let service = ctx.data::<QcService>()?;
        let material = service.get_qc_material_by_code(material_code).await?;
        Ok(Some(material))
    }

    /// List QC materials with filtering and pagination
    async fn qc_materials(
        &self,
        ctx: &Context<'_>,
        filter: QcMaterialFilter,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<QcMaterialPaginated> {
        let service = ctx.data::<QcService>()?;

        let pagination = PaginationParams {
            page: page.unwrap_or(1) as u32,
            page_size: page_size.unwrap_or(20) as u32,
        };

        let paginated = service.list_qc_materials(filter, pagination).await.map_err(|e| e.extend())?;

        Ok(QcMaterialPaginated {
            data: paginated.edges.into_iter().map(|edge| edge.node).collect(),
            total: paginated.page_info.total_count as i32,
            page: paginated.page_info.current_page as i32,
            page_size: paginated.page_info.page_size as i32,
            total_pages: paginated.page_info.total_pages as i32,
        })
    }

    /// Get QC rule by ID
    async fn qc_rule(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcRule>> {
        let service = ctx.data::<QcService>()?;
        let rule = service.get_qc_rule(id).await?;
        Ok(Some(rule))
    }

    /// List QC rules for organization
    async fn qc_rules(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
    ) -> Result<Vec<QcRule>> {
        let service = ctx.data::<QcService>()?;
        Ok(service.list_qc_rules(organization_id).await.map_err(|e| e.extend())?)
    }

    /// Get QC result by ID
    async fn qc_result(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcResult>> {
        let service = ctx.data::<QcService>()?;
        let result = service.get_qc_result(id).await?;
        Ok(Some(result))
    }

    /// List QC results with filtering
    async fn qc_results(
        &self,
        ctx: &Context<'_>,
        filter: QcResultFilter,
    ) -> Result<Vec<QcResult>> {
        let service = ctx.data::<QcService>()?;
        Ok(service.list_qc_results(filter).await.map_err(|e| e.extend())?)
    }

    /// Get QC violation by ID
    async fn qc_violation(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcViolation>> {
        let service = ctx.data::<QcService>()?;
        let violation = service.get_violation(id).await?;
        Ok(Some(violation))
    }

    /// List QC violations with filtering
    async fn qc_violations(
        &self,
        ctx: &Context<'_>,
        filter: QcViolationFilter,
    ) -> Result<Vec<QcViolation>> {
        let service = ctx.data::<QcService>()?;
        Ok(service.list_violations(filter).await.map_err(|e| e.extend())?)
    }

    /// Get corrective action by ID
    async fn corrective_action(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcCorrectiveAction>> {
        let service = ctx.data::<QcService>()?;
        let action = service.get_corrective_action(id).await?;
        Ok(Some(action))
    }

    /// List corrective actions for violation
    async fn corrective_actions(
        &self,
        ctx: &Context<'_>,
        qc_violation_id: Uuid,
    ) -> Result<Vec<QcCorrectiveAction>> {
        let service = ctx.data::<QcService>()?;
        Ok(service.list_corrective_actions(qc_violation_id).await.map_err(|e| e.extend())?)
    }

    /// Get external QC program by ID
    async fn external_program(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<QcExternalProgram>> {
        let service = ctx.data::<QcService>()?;
        let program = service.get_external_program(id).await?;
        Ok(Some(program))
    }

    /// List external QC programs for organization
    async fn external_programs(
        &self,
        ctx: &Context<'_>,
        organization_id: Uuid,
    ) -> Result<Vec<QcExternalProgram>> {
        let service = ctx.data::<QcService>()?;
        Ok(service.list_external_programs(organization_id).await.map_err(|e| e.extend())?)
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create QC material
    async fn create_qc_material(
        &self,
        ctx: &Context<'_>,
        input: CreateQcMaterialInput,
    ) -> Result<QcMaterial> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_qc_material(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update QC material
    async fn update_qc_material(
        &self,
        ctx: &Context<'_>,
        input: UpdateQcMaterialInput,
    ) -> Result<QcMaterial> {
        let service = ctx.data::<QcService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_qc_material(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Delete QC material
    async fn delete_qc_material(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<QcService>()?;
        Ok(service.delete_qc_material(id).await.map_err(|e| e.extend())?)
    }

    /// Create QC rule
    async fn create_qc_rule(
        &self,
        ctx: &Context<'_>,
        input: CreateQcRuleInput,
    ) -> Result<QcRule> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_qc_rule(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Assign rule to QC material
    async fn assign_rule_to_material(
        &self,
        ctx: &Context<'_>,
        input: AssignRuleToMaterialInput,
    ) -> Result<QcMaterialRule> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.assign_rule_to_material(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Unassign rule from QC material
    async fn unassign_rule_from_material(
        &self,
        ctx: &Context<'_>,
        qc_material_id: Uuid,
        qc_rule_id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<QcService>()?;
        Ok(service.unassign_rule_from_material(qc_material_id, qc_rule_id).await.map_err(|e| e.extend())?)
    }

    /// Record QC result
    async fn record_qc_result(
        &self,
        ctx: &Context<'_>,
        input: RecordQcResultInput,
    ) -> Result<QcResult> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.record_qc_result(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Review QC result
    async fn review_qc_result(
        &self,
        ctx: &Context<'_>,
        input: ReviewQcResultInput,
    ) -> Result<QcResult> {
        let service = ctx.data::<QcService>()?;

        // In production, get reviewed_by from authenticated user context
        let reviewed_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.review_qc_result(input, reviewed_by).await.map_err(|e| e.extend())?)
    }

    /// Acknowledge violation
    async fn acknowledge_violation(
        &self,
        ctx: &Context<'_>,
        input: AcknowledgeViolationInput,
    ) -> Result<QcViolation> {
        let service = ctx.data::<QcService>()?;

        // In production, get acknowledged_by from authenticated user context
        let acknowledged_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.acknowledge_violation(input, acknowledged_by).await.map_err(|e| e.extend())?)
    }

    /// Resolve violation
    async fn resolve_violation(
        &self,
        ctx: &Context<'_>,
        input: ResolveViolationInput,
    ) -> Result<QcViolation> {
        let service = ctx.data::<QcService>()?;

        // In production, get resolved_by from authenticated user context
        let resolved_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.resolve_violation(input, resolved_by).await.map_err(|e| e.extend())?)
    }

    /// Create corrective action
    async fn create_corrective_action(
        &self,
        ctx: &Context<'_>,
        input: CreateCorrectiveActionInput,
    ) -> Result<QcCorrectiveAction> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_corrective_action(input, created_by).await.map_err(|e| e.extend())?)
    }

    /// Update corrective action
    async fn update_corrective_action(
        &self,
        ctx: &Context<'_>,
        input: UpdateCorrectiveActionInput,
    ) -> Result<QcCorrectiveAction> {
        let service = ctx.data::<QcService>()?;

        // In production, get updated_by from authenticated user context
        let updated_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.update_corrective_action(input, updated_by).await.map_err(|e| e.extend())?)
    }

    /// Create external QC program
    async fn create_external_program(
        &self,
        ctx: &Context<'_>,
        input: CreateExternalProgramInput,
    ) -> Result<QcExternalProgram> {
        let service = ctx.data::<QcService>()?;

        // In production, get created_by from authenticated user context
        let created_by = Uuid::new_v4(); // TODO: Replace with actual user ID from JWT

        Ok(service.create_external_program(input, created_by).await.map_err(|e| e.extend())?)
    }
}

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(async_graphql::SimpleObject)]
pub struct QcMaterialPaginated {
    pub data: Vec<QcMaterial>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}
