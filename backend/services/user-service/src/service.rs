use uuid::Uuid;
use chrono::{Duration, Utc};
use common::error::{Error, Result};

use crate::domain::*;
use crate::repository::*;
use crate::organization_client::{OrganizationClient, CreateOrganizationInput};

// ============================================================================
// User Service
// ============================================================================

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    role_repo: RoleRepository,
    permission_repo: PermissionRepository,
    user_role_repo: UserRoleRepository,
    session_repo: SessionRepository,
    activity_repo: ActivityLogRepository,
    org_client: OrganizationClient,
}

impl UserService {
    pub fn new(
        user_repo: UserRepository,
        role_repo: RoleRepository,
        permission_repo: PermissionRepository,
        user_role_repo: UserRoleRepository,
        session_repo: SessionRepository,
        activity_repo: ActivityLogRepository,
        org_client: OrganizationClient,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            permission_repo,
            user_role_repo,
            session_repo,
            activity_repo,
            org_client,
        }
    }

    // ========================================================================
    // Authentication Operations
    // ========================================================================

    pub async fn register(&self, input: RegisterUserInput) -> Result<User> {
        input.validate()?;

        // Check if email already exists
        if let Some(_) = self.user_repo.find_by_email(&input.email).await? {
            return Err(Error::AlreadyExists("Email already registered".to_string()));
        }

        // Check if mobile already exists (if provided)
        if let Some(ref mobile) = input.mobile_number {
            if let Some(_) = self.user_repo.find_by_mobile(mobile).await? {
                return Err(Error::AlreadyExists("Mobile number already registered".to_string()));
            }
        }

        // Hash password
        let password_hash = self.hash_password(&input.password)?;

        // Create user
        let user = self.user_repo.create(input, password_hash).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "USER_REGISTERED",
            "AUTH",
            Some(format!("User registered: {}", user.email)),
            None,
        ).await?;

        // TODO: Send email verification
        // TODO: Publish USER_REGISTERED event

        tracing::info!("User registered: {}", user.email);
        Ok(user)
    }

    /// Register a new organization admin - creates both organization and user atomically
    pub async fn register_org_admin(
        &self,
        user_input: RegisterUserInput,
        org_name: String,
        org_phone: Option<String>,
    ) -> Result<(User, Uuid)> {
        // Validate user input first
        user_input.validate()?;

        // Validate organization name
        if org_name.trim().is_empty() {
            return Err(Error::Validation("Organization name is required".to_string()));
        }

        // Check if email already exists
        if let Some(_) = self.user_repo.find_by_email(&user_input.email).await? {
            return Err(Error::AlreadyExists("Email already registered".to_string()));
        }

        // Check if mobile already exists (if provided)
        if let Some(ref mobile) = user_input.mobile_number {
            if let Some(_) = self.user_repo.find_by_mobile(mobile).await? {
                return Err(Error::AlreadyExists("Mobile number already registered".to_string()));
            }
        }

        tracing::info!("Creating organization: {}", org_name);

        // Step 1: Create organization via organization-service
        let org_data = self.org_client.create_organization(
            CreateOrganizationInput {
                organization_name: org_name.clone(),
                legal_name: None,
                organization_type: "CLINICAL_LAB".to_string(), // Default type
                email: user_input.email.clone(),
                phone: org_phone,
            }
        ).await?;

        let organization_id = Uuid::parse_str(&org_data.id)
            .map_err(|e| Error::Validation(format!("Invalid organization ID: {}", e)))?;

        tracing::info!("Organization created successfully: {}", organization_id);

        // Step 2: Hash password
        let password_hash = self.hash_password(&user_input.password)?;

        // Step 3: Create user with organization_id
        let mut user_with_org = user_input.clone();
        user_with_org.organization_id = Some(organization_id);

        tracing::info!("Creating user for organization: {}", organization_id);

        let user = match self.user_repo.create(user_with_org, password_hash).await {
            Ok(user) => user,
            Err(e) => {
                // ROLLBACK: Delete the organization if user creation failed
                tracing::error!("User creation failed, rolling back organization {}: {}", organization_id, e);
                if let Err(rollback_err) = self.org_client.delete_organization(organization_id).await {
                    tracing::error!("Failed to rollback organization {}: {}", organization_id, rollback_err);
                }
                return Err(e);
            }
        };

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            Some(organization_id),
            "ORG_ADMIN_REGISTERED",
            "AUTH",
            Some(format!("Organization admin registered: {} for org: {}", user.email, org_name)),
            None,
        ).await?;

        tracing::info!("Organization admin registered successfully: {} (org: {})", user.email, organization_id);

        Ok((user, organization_id))
    }

    pub async fn login(&self, input: LoginInput, ip_address: Option<String>) -> Result<LoginResponse> {
        input.validate()?;

        // Find user by email
        let user = self.user_repo
            .find_by_email(&input.email)
            .await?
            .ok_or_else(|| Error::AuthenticationFailed("Invalid credentials".to_string()))?;

        // Check if user can login - provide specific error messages
        if user.is_deleted {
            return Err(Error::AuthenticationFailed(
                "This account has been deleted. Please contact support.".to_string()
            ));
        }

        if user.is_locked() {
            let locked_until = user.locked_until.unwrap();
            return Err(Error::AuthenticationFailed(
                format!("Account is temporarily locked until {}. Please try again later or contact support.",
                    locked_until.format("%Y-%m-%d %H:%M UTC"))
            ));
        }

        if user.user_status != UserStatus::Active {
            let message = match user.user_status {
                UserStatus::PendingVerification =>
                    "Your account is pending verification. Please check your email for the verification link.".to_string(),
                UserStatus::Inactive =>
                    "Your account is inactive. Please contact your administrator.".to_string(),
                UserStatus::Suspended =>
                    "Your account has been suspended. Please contact support for assistance.".to_string(),
                UserStatus::Locked =>
                    "Your account has been locked. Please contact support to unlock your account.".to_string(),
                _ => "Account is not active".to_string(),
            };
            return Err(Error::AuthenticationFailed(message));
        }

        if !user.email_verified {
            return Err(Error::AuthenticationFailed(
                "Please verify your email address before logging in. Check your inbox for the verification link or request a new one.".to_string()
            ));
        }

        // Verify password
        if !self.verify_password(&input.password, &user.password_hash)? {
            // Record failed attempt
            self.user_repo.record_login_attempt(user.id, false, ip_address.clone()).await?;

            return Err(Error::AuthenticationFailed("Invalid credentials".to_string()));
        }

        // Check if account is locked
        if user.is_locked() {
            return Err(Error::AuthenticationFailed("Account is locked due to multiple failed login attempts".to_string()));
        }

        // Generate tokens
        let session_token = self.generate_token();
        let access_token = self.generate_jwt(&user, Duration::hours(1))?;
        let refresh_token = self.generate_token();

        // Create session
        let expires_at = Utc::now() + Duration::hours(1);
        let refresh_expires_at = Utc::now() + Duration::days(30);

        let session = self.session_repo.create(
            user.id,
            session_token,
            access_token.clone(),
            refresh_token.clone(),
            (input.device_id, input.device_name, Some("WEB".to_string())),
            ip_address.clone(),
            expires_at,
            refresh_expires_at,
        ).await?;

        // Record successful login
        self.user_repo.record_login_attempt(user.id, true, ip_address.clone()).await?;

        // Get user permissions
        let permissions = self.get_user_permission_codes(user.id).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            Some(session.id),
            "LOGIN",
            "AUTH",
            Some("User logged in".to_string()),
            ip_address,
        ).await?;

        // TODO: Publish USER_LOGGED_IN event

        tracing::info!("User logged in: {}", user.email);

        Ok(LoginResponse {
            user,
            access_token,
            refresh_token,
            expires_in: 3600, // 1 hour
            permissions,
        })
    }

    pub async fn logout(&self, session_token: &str) -> Result<()> {
        let session = self.session_repo
            .find_by_token(session_token)
            .await?
            .ok_or_else(|| Error::NotFound("Session not found".to_string()))?;

        self.session_repo.logout(session.id).await?;

        // Log activity
        self.activity_repo.log(
            Some(session.user_id),
            Some(session.id),
            "LOGOUT",
            "AUTH",
            Some("User logged out".to_string()),
            None,
        ).await?;

        tracing::info!("User logged out");
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<RefreshTokenResponse> {
        let session = self.session_repo
            .find_by_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| Error::AuthenticationFailed("Invalid refresh token".to_string()))?;

        if !session.is_refresh_token_valid() {
            return Err(Error::AuthenticationFailed("Refresh token expired".to_string()));
        }

        // Get user
        let user = self.get_user(session.user_id).await?;

        // Generate new tokens
        let new_access_token = self.generate_jwt(&user, Duration::hours(1))?;
        let new_refresh_token = self.generate_token();

        // Update session (in production, you'd want to update the session record)

        tracing::info!("Token refreshed for user: {}", user.email);

        Ok(RefreshTokenResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in: 3600,
        })
    }

    // ========================================================================
    // User Operations
    // ========================================================================

    pub async fn get_user(&self, id: Uuid) -> Result<User> {
        self.user_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User not found: {}", id)))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User not found: {}", email)))
    }

    pub async fn search_users(&self, filter: UserFilter, limit: i64) -> Result<Vec<User>> {
        self.user_repo.search(filter, limit).await
    }

    pub async fn update_user(&self, input: UpdateUserInput) -> Result<User> {
        let user = self.user_repo.update(input).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "USER_UPDATED",
            "USER",
            Some("User profile updated".to_string()),
            None,
        ).await?;

        tracing::info!("User updated: {}", user.email);
        Ok(user)
    }

    pub async fn update_user_status(&self, user_id: Uuid, status: UserStatus) -> Result<User> {
        let user = self.user_repo.update_status(user_id, status.clone()).await?;

        // If suspended or locked, revoke all sessions
        if matches!(status, UserStatus::Suspended | UserStatus::Locked) {
            self.session_repo.revoke_user_sessions(user_id).await?;
        }

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "STATUS_CHANGED",
            "USER",
            Some(format!("User status changed to: {:?}", status)),
            None,
        ).await?;

        tracing::info!("User status updated: {} -> {:?}", user.email, status);
        Ok(user)
    }

    // ========================================================================
    // Password Operations
    // ========================================================================

    pub async fn change_password(&self, input: ChangePasswordInput) -> Result<User> {
        input.validate()?;

        // Get user
        let user = self.get_user(input.user_id).await?;

        // Verify current password
        if !self.verify_password(&input.current_password, &user.password_hash)? {
            return Err(Error::AuthenticationFailed("Current password is incorrect".to_string()));
        }

        // Hash new password
        let password_hash = self.hash_password(&input.new_password)?;

        // Update password
        let user = self.user_repo.update_password(input.user_id, password_hash).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "PASSWORD_CHANGED",
            "AUTH",
            Some("Password changed successfully".to_string()),
            None,
        ).await?;

        tracing::info!("Password changed for user: {}", user.email);
        Ok(user)
    }

    pub async fn request_password_reset(&self, input: ResetPasswordInput) -> Result<()> {
        input.validate()?;

        // Find user
        let user = self.user_repo.find_by_email(&input.email).await?;

        if let Some(user) = user {
            // Generate reset token
            let token = self.generate_token();
            let expires_at = Utc::now() + Duration::hours(1);

            // Store token
            self.user_repo.set_password_reset_token(user.id, token.clone(), expires_at).await?;

            // TODO: Send password reset email

            tracing::info!("Password reset requested for: {}", user.email);
        } else {
            // Don't reveal if email exists or not
            tracing::warn!("Password reset requested for non-existent email: {}", input.email);
        }

        Ok(())
    }

    pub async fn confirm_password_reset(&self, input: ConfirmPasswordResetInput) -> Result<User> {
        input.validate()?;

        // Find user by token
        let user = self.user_repo
            .find_by_reset_token(&input.token)
            .await?
            .ok_or_else(|| Error::NotFound("Invalid or expired reset token".to_string()))?;

        // Hash new password
        let password_hash = self.hash_password(&input.new_password)?;

        // Update password
        let user = self.user_repo.update_password(user.id, password_hash).await?;

        // Clear reset token
        self.user_repo.clear_reset_token(user.id).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "PASSWORD_RESET",
            "AUTH",
            Some("Password reset successfully".to_string()),
            None,
        ).await?;

        tracing::info!("Password reset for user: {}", user.email);
        Ok(user)
    }

    // ========================================================================
    // Role and Permission Operations
    // ========================================================================

    pub async fn create_role(&self, input: CreateRoleInput, user_id: Uuid) -> Result<Role> {
        input.validate()?;

        let role = self.role_repo.create(input, user_id).await?;

        tracing::info!("Role created: {}", role.role_code);
        Ok(role)
    }

    pub async fn get_all_roles(&self, org_id: Option<Uuid>) -> Result<Vec<Role>> {
        self.role_repo.get_all(org_id).await
    }

    pub async fn get_all_permissions(&self) -> Result<Vec<Permission>> {
        self.permission_repo.get_all().await
    }

    pub async fn assign_role_to_user(&self, input: AssignRoleInput, assigned_by: Uuid) -> Result<()> {
        // Verify role exists
        let _ = self.role_repo
            .find_by_id(input.role_id)
            .await?
            .ok_or_else(|| Error::NotFound("Role not found".to_string()))?;

        // Assign role
        self.user_role_repo.assign_role(input.clone(), assigned_by).await?;

        // Log activity
        self.activity_repo.log(
            Some(input.user_id),
            None,
            "ROLE_ASSIGNED",
            "USER",
            Some(format!("Role assigned to user")),
            None,
        ).await?;

        tracing::info!("Role assigned to user");
        Ok(())
    }

    pub async fn remove_role_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        self.user_role_repo.remove_role(user_id, role_id).await?;

        // Log activity
        self.activity_repo.log(
            Some(user_id),
            None,
            "ROLE_REMOVED",
            "USER",
            Some("Role removed from user".to_string()),
            None,
        ).await?;

        tracing::info!("Role removed from user");
        Ok(())
    }

    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>> {
        self.user_role_repo.get_user_roles(user_id).await
    }

    pub async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<Permission>> {
        self.user_role_repo.get_user_permissions(user_id).await
    }

    pub async fn get_user_permission_codes(&self, user_id: Uuid) -> Result<Vec<String>> {
        let permissions = self.get_user_permissions(user_id).await?;
        Ok(permissions.into_iter().map(|p| p.permission_code).collect())
    }

    pub async fn has_permission(&self, user_id: Uuid, permission_code: &str) -> Result<bool> {
        let permissions = self.get_user_permission_codes(user_id).await?;
        Ok(permissions.contains(&permission_code.to_string()))
    }

    // ========================================================================
    // Session Operations
    // ========================================================================

    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<UserSession>> {
        self.session_repo.get_user_sessions(user_id).await
    }

    pub async fn revoke_user_sessions(&self, user_id: Uuid) -> Result<()> {
        self.session_repo.revoke_user_sessions(user_id).await?;

        tracing::info!("All sessions revoked for user");
        Ok(())
    }

    // ========================================================================
    // Activity Operations
    // ========================================================================

    pub async fn get_user_activity(&self, user_id: Uuid, limit: i64) -> Result<Vec<ActivityLog>> {
        self.activity_repo.get_user_activity(user_id, limit).await
    }

    // ========================================================================
    // Email Verification
    // ========================================================================

    pub async fn verify_email(&self, user_id: Uuid) -> Result<User> {
        let user = self.user_repo.verify_email(user_id).await?;

        // Log activity
        self.activity_repo.log(
            Some(user.id),
            None,
            "EMAIL_VERIFIED",
            "AUTH",
            Some("Email verified successfully".to_string()),
            None,
        ).await?;

        tracing::info!("Email verified for user: {}", user.email);
        Ok(user)
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    fn hash_password(&self, password: &str) -> Result<String> {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::Custom(format!("Failed to hash password: {}", e)))?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };

        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| Error::Custom(format!("Failed to parse hash: {}", e)))?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    fn generate_token(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = rand::thread_rng();

        (0..64)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    fn generate_jwt(&self, user: &User, duration: Duration) -> Result<String> {
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: i64,
            iat: i64,
            user_id: String,
            email: String,
            user_type: String,
        }

        let now = Utc::now();
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (now + duration).timestamp(),
            iat: now.timestamp(),
            user_id: user.id.to_string(),
            email: user.email.clone(),
            user_type: format!("{:?}", user.user_type),
        };

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| Error::Custom(format!("Failed to generate JWT: {}", e)))?;

        Ok(token)
    }
}
