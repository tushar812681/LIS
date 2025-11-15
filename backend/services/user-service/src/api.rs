use async_graphql::{Context, Object, Result, SimpleObject, InputObject, Enum, ID};
use uuid::Uuid;

use crate::domain::*;
use crate::service::UserService;

// ============================================================================
// GraphQL Types
// ============================================================================

#[derive(SimpleObject)]
pub struct UserGQL {
    pub id: ID,
    pub user_code: String,
    pub organization_id: Option<ID>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile_number: Option<String>,
    pub user_type: UserTypeEnum,
    pub user_status: UserStatusEnum,
    pub professional_title: Option<String>,
    pub qualification: Option<String>,
    pub department: Option<String>,
    pub email_verified: bool,
    pub mobile_verified: bool,
    pub two_factor_enabled: bool,
    pub created_at: String,
}

impl From<User> for UserGQL {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string().into(),
            user_code: user.user_code,
            organization_id: user.organization_id.map(|id| id.to_string().into()),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            mobile_number: user.mobile_number,
            user_type: user.user_type.into(),
            user_status: user.user_status.into(),
            professional_title: user.professional_title,
            qualification: user.qualification,
            department: user.department,
            email_verified: user.email_verified,
            mobile_verified: user.mobile_verified,
            two_factor_enabled: user.two_factor_enabled,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct RoleGQL {
    pub id: ID,
    pub role_code: String,
    pub role_name: String,
    pub description: Option<String>,
    pub is_system_role: bool,
}

impl From<Role> for RoleGQL {
    fn from(role: Role) -> Self {
        Self {
            id: role.id.to_string().into(),
            role_code: role.role_code,
            role_name: role.role_name,
            description: role.description,
            is_system_role: role.is_system_role,
        }
    }
}

#[derive(SimpleObject)]
pub struct PermissionGQL {
    pub id: ID,
    pub permission_code: String,
    pub permission_name: String,
    pub module: String,
    pub action: String,
}

impl From<Permission> for PermissionGQL {
    fn from(permission: Permission) -> Self {
        Self {
            id: permission.id.to_string().into(),
            permission_code: permission.permission_code,
            permission_name: permission.permission_name,
            module: permission.module,
            action: permission.action,
        }
    }
}

#[derive(SimpleObject)]
pub struct LoginResponseGQL {
    pub user: UserGQL,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub permissions: Vec<String>,
}

impl From<LoginResponse> for LoginResponseGQL {
    fn from(response: LoginResponse) -> Self {
        Self {
            user: response.user.into(),
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in as i32,
            permissions: response.permissions,
        }
    }
}

// ============================================================================
// Enums
// ============================================================================

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UserTypeEnum {
    SuperAdmin,
    OrgAdmin,
    Manager,
    Doctor,
    Technician,
    Nurse,
    Receptionist,
    BillingStaff,
    LabAssistant,
    QualityManager,
    Patient,
}

impl From<UserType> for UserTypeEnum {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::SuperAdmin => UserTypeEnum::SuperAdmin,
            UserType::OrgAdmin => UserTypeEnum::OrgAdmin,
            UserType::Manager => UserTypeEnum::Manager,
            UserType::Doctor => UserTypeEnum::Doctor,
            UserType::Technician => UserTypeEnum::Technician,
            UserType::Nurse => UserTypeEnum::Nurse,
            UserType::Receptionist => UserTypeEnum::Receptionist,
            UserType::BillingStaff => UserTypeEnum::BillingStaff,
            UserType::LabAssistant => UserTypeEnum::LabAssistant,
            UserType::QualityManager => UserTypeEnum::QualityManager,
            UserType::Patient => UserTypeEnum::Patient,
        }
    }
}

impl From<UserTypeEnum> for UserType {
    fn from(user_type: UserTypeEnum) -> Self {
        match user_type {
            UserTypeEnum::SuperAdmin => UserType::SuperAdmin,
            UserTypeEnum::OrgAdmin => UserType::OrgAdmin,
            UserTypeEnum::Manager => UserType::Manager,
            UserTypeEnum::Doctor => UserType::Doctor,
            UserTypeEnum::Technician => UserType::Technician,
            UserTypeEnum::Nurse => UserType::Nurse,
            UserTypeEnum::Receptionist => UserType::Receptionist,
            UserTypeEnum::BillingStaff => UserType::BillingStaff,
            UserTypeEnum::LabAssistant => UserType::LabAssistant,
            UserTypeEnum::QualityManager => UserType::QualityManager,
            UserTypeEnum::Patient => UserType::Patient,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UserStatusEnum {
    Active,
    Inactive,
    Suspended,
    Locked,
    PendingVerification,
}

impl From<UserStatus> for UserStatusEnum {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Active => UserStatusEnum::Active,
            UserStatus::Inactive => UserStatusEnum::Inactive,
            UserStatus::Suspended => UserStatusEnum::Suspended,
            UserStatus::Locked => UserStatusEnum::Locked,
            UserStatus::PendingVerification => UserStatusEnum::PendingVerification,
        }
    }
}

impl From<UserStatusEnum> for UserStatus {
    fn from(status: UserStatusEnum) -> Self {
        match status {
            UserStatusEnum::Active => UserStatus::Active,
            UserStatusEnum::Inactive => UserStatus::Inactive,
            UserStatusEnum::Suspended => UserStatus::Suspended,
            UserStatusEnum::Locked => UserStatus::Locked,
            UserStatusEnum::PendingVerification => UserStatus::PendingVerification,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

#[derive(InputObject)]
pub struct RegisterUserInputGQL {
    pub organization_id: Option<ID>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile_number: Option<String>,
    pub password: String,
    pub user_type: UserTypeEnum,
    pub department: Option<String>,
}

impl TryFrom<RegisterUserInputGQL> for RegisterUserInput {
    type Error = String;

    fn try_from(input: RegisterUserInputGQL) -> std::result::Result<Self, Self::Error> {
        let organization_id = if let Some(id) = input.organization_id {
            Some(Uuid::parse_str(&id).map_err(|e| format!("Invalid organization_id: {}", e))?)
        } else {
            None
        };

        Ok(RegisterUserInput {
            organization_id,
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            mobile_number: input.mobile_number,
            password: input.password,
            user_type: input.user_type.into(),
            department: input.department,
        })
    }
}

#[derive(InputObject)]
pub struct RegisterOrgAdminInputGQL {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub mobile_number: Option<String>,
    pub password: String,
    pub organization_name: String,
    pub organization_phone: Option<String>,
}

#[derive(InputObject)]
pub struct LoginInputGQL {
    pub email: String,
    pub password: String,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
}

impl From<LoginInputGQL> for LoginInput {
    fn from(input: LoginInputGQL) -> Self {
        LoginInput {
            email: input.email,
            password: input.password,
            device_id: input.device_id,
            device_name: input.device_name,
        }
    }
}

#[derive(InputObject)]
pub struct ChangePasswordInputGQL {
    pub user_id: ID,
    pub current_password: String,
    pub new_password: String,
}

impl TryFrom<ChangePasswordInputGQL> for ChangePasswordInput {
    type Error = String;

    fn try_from(input: ChangePasswordInputGQL) -> std::result::Result<Self, Self::Error> {
        let user_id = Uuid::parse_str(&input.user_id)
            .map_err(|e| format!("Invalid user_id: {}", e))?;

        Ok(ChangePasswordInput {
            user_id,
            current_password: input.current_password,
            new_password: input.new_password,
        })
    }
}

#[derive(InputObject)]
pub struct CreateRoleInputGQL {
    pub role_code: String,
    pub role_name: String,
    pub description: Option<String>,
    pub organization_id: Option<ID>,
    pub permission_ids: Vec<ID>,
}

impl TryFrom<CreateRoleInputGQL> for CreateRoleInput {
    type Error = String;

    fn try_from(input: CreateRoleInputGQL) -> std::result::Result<Self, Self::Error> {
        let organization_id = if let Some(id) = input.organization_id {
            Some(Uuid::parse_str(&id).map_err(|e| format!("Invalid organization_id: {}", e))?)
        } else {
            None
        };

        let permission_ids: std::result::Result<Vec<Uuid>, String> = input
            .permission_ids
            .iter()
            .map(|id| Uuid::parse_str(id).map_err(|e| format!("Invalid permission_id: {}", e)))
            .collect();

        Ok(CreateRoleInput {
            role_code: input.role_code,
            role_name: input.role_name,
            description: input.description,
            organization_id,
            permission_ids: permission_ids?,
        })
    }
}

// ============================================================================
// GraphQL Query Root
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get current user (from auth context)
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserGQL>> {
        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let service = ctx.data::<UserService>()?;
        match service.get_user(user_id).await {
            Ok(user) => Ok(Some(user.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserGQL>> {
        let service = ctx.data::<UserService>()?;
        let user_id = Uuid::parse_str(&id)?;

        match service.get_user(user_id).await {
            Ok(user) => Ok(Some(user.into())),
            Err(_) => Ok(None),
        }
    }

    /// Get user by email
    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<UserGQL>> {
        let service = ctx.data::<UserService>()?;

        match service.get_user_by_email(&email).await {
            Ok(user) => Ok(Some(user.into())),
            Err(_) => Ok(None),
        }
    }

    /// Search users
    async fn search_users(
        &self,
        ctx: &Context<'_>,
        search_query: Option<String>,
        user_type: Option<UserTypeEnum>,
        user_status: Option<UserStatusEnum>,
        limit: Option<i32>,
    ) -> Result<Vec<UserGQL>> {
        let service = ctx.data::<UserService>()?;

        let filter = UserFilter {
            organization_id: None, // TODO: Get from auth context
            user_type: user_type.map(|t| t.into()),
            user_status: user_status.map(|s| s.into()),
            department: None,
            search_query,
        };

        let users = service.search_users(filter, limit.unwrap_or(50) as i64).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    /// Get all roles
    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<RoleGQL>> {
        let service = ctx.data::<UserService>()?;
        let roles = service.get_all_roles(None).await?;
        Ok(roles.into_iter().map(|r| r.into()).collect())
    }

    /// Get all permissions
    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<PermissionGQL>> {
        let service = ctx.data::<UserService>()?;
        let permissions = service.get_all_permissions().await?;
        Ok(permissions.into_iter().map(|p| p.into()).collect())
    }

    /// Get user roles
    async fn user_roles(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<RoleGQL>> {
        let service = ctx.data::<UserService>()?;
        let id = Uuid::parse_str(&user_id)?;
        let roles = service.get_user_roles(id).await?;
        Ok(roles.into_iter().map(|r| r.into()).collect())
    }

    /// Get user permissions
    async fn user_permissions(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<PermissionGQL>> {
        let service = ctx.data::<UserService>()?;
        let id = Uuid::parse_str(&user_id)?;
        let permissions = service.get_user_permissions(id).await?;
        Ok(permissions.into_iter().map(|p| p.into()).collect())
    }
}

// ============================================================================
// GraphQL Mutation Root
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Register new user
    async fn register(&self, ctx: &Context<'_>, input: RegisterUserInputGQL) -> Result<UserGQL> {
        let service = ctx.data::<UserService>()?;

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let user = service.register(domain_input).await?;
        Ok(user.into())
    }

    /// Register new organization admin (creates organization + user atomically)
    async fn register_org_admin(&self, ctx: &Context<'_>, input: RegisterOrgAdminInputGQL) -> Result<UserGQL> {
        let service = ctx.data::<UserService>()?;

        let user_input = RegisterUserInput {
            organization_id: None, // Will be set by service after creating org
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            mobile_number: input.mobile_number,
            password: input.password,
            user_type: UserType::OrgAdmin,
            department: None,
        };

        let (user, org_id) = service.register_org_admin(
            user_input,
            input.organization_name,
            input.organization_phone,
        ).await?;

        tracing::info!("Organization admin created: user={}, org={}", user.id, org_id);

        Ok(user.into())
    }

    /// Login
    async fn login(&self, ctx: &Context<'_>, input: LoginInputGQL) -> Result<LoginResponseGQL> {
        let service = ctx.data::<UserService>()?;

        // TODO: Get IP address from request context
        let ip_address = None;

        let response = service.login(input.into(), ip_address).await?;
        Ok(response.into())
    }

    /// Logout
    async fn logout(&self, ctx: &Context<'_>, session_token: String) -> Result<bool> {
        let service = ctx.data::<UserService>()?;
        service.logout(&session_token).await?;
        Ok(true)
    }

    /// Refresh token
    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> Result<LoginResponseGQL> {
        let service = ctx.data::<UserService>()?;
        let response = service.refresh_token(&refresh_token).await?;

        // Convert RefreshTokenResponse to LoginResponseGQL
        // For simplicity, we'll return a basic structure
        // In production, you'd want to get the full user details
        Err(async_graphql::Error::new("Not implemented - use dedicated refresh endpoint"))
    }

    /// Change password
    async fn change_password(&self, ctx: &Context<'_>, input: ChangePasswordInputGQL) -> Result<bool> {
        let service = ctx.data::<UserService>()?;

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        service.change_password(domain_input).await?;
        Ok(true)
    }

    /// Request password reset
    async fn request_password_reset(&self, ctx: &Context<'_>, email: String) -> Result<bool> {
        let service = ctx.data::<UserService>()?;
        service.request_password_reset(ResetPasswordInput { email }).await?;
        Ok(true)
    }

    /// Update user status
    async fn update_user_status(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        status: UserStatusEnum,
    ) -> Result<UserGQL> {
        let service = ctx.data::<UserService>()?;
        let id = Uuid::parse_str(&user_id)?;

        let user = service.update_user_status(id, status.into()).await?;
        Ok(user.into())
    }

    /// Create role
    async fn create_role(&self, ctx: &Context<'_>, input: CreateRoleInputGQL) -> Result<RoleGQL> {
        let service = ctx.data::<UserService>()?;

        // TODO: Get user_id from auth context
        let user_id = Uuid::nil();

        let domain_input = input.try_into()
            .map_err(|e: String| async_graphql::Error::new(e))?;

        let role = service.create_role(domain_input, user_id).await?;
        Ok(role.into())
    }

    /// Assign role to user
    async fn assign_role(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        role_id: ID,
    ) -> Result<bool> {
        let service = ctx.data::<UserService>()?;

        // TODO: Get assigned_by from auth context
        let assigned_by = Uuid::nil();

        let input = AssignRoleInput {
            user_id: Uuid::parse_str(&user_id)?,
            role_id: Uuid::parse_str(&role_id)?,
        };

        service.assign_role_to_user(input, assigned_by).await?;
        Ok(true)
    }

    /// Remove role from user
    async fn remove_role(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        role_id: ID,
    ) -> Result<bool> {
        let service = ctx.data::<UserService>()?;

        let uid = Uuid::parse_str(&user_id)?;
        let rid = Uuid::parse_str(&role_id)?;

        service.remove_role_from_user(uid, rid).await?;
        Ok(true)
    }

    /// Verify email
    async fn verify_email(&self, ctx: &Context<'_>, user_id: ID) -> Result<UserGQL> {
        let service = ctx.data::<UserService>()?;
        let id = Uuid::parse_str(&user_id)?;

        let user = service.verify_email(id).await?;
        Ok(user.into())
    }
}
