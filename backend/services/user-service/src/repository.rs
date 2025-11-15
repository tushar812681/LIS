use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use common::error::{Error, Result};

use crate::domain::*;

// ============================================================================
// User Repository
// ============================================================================

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: RegisterUserInput, password_hash: String) -> Result<User> {
        input.validate()?;

        let user_code = self.generate_user_code(&input.user_type).await?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, user_code, organization_id, first_name, last_name,
                email, mobile_number, password_hash, user_type, user_status,
                department, email_verified, mobile_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, FALSE, FALSE)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&user_code)
        .bind(input.organization_id)
        .bind(&input.first_name)
        .bind(&input.last_name)
        .bind(&input.email)
        .bind(&input.mobile_number)
        .bind(&password_hash)
        .bind(&input.user_type)
        .bind(UserStatus::PendingVerification)
        .bind(&input.department)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_deleted = FALSE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE LOWER(email) = LOWER($1) AND is_deleted = FALSE"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn find_by_user_code(&self, user_code: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE user_code = $1 AND is_deleted = FALSE"
        )
        .bind(user_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn find_by_mobile(&self, mobile_number: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE mobile_number = $1 AND is_deleted = FALSE"
        )
        .bind(mobile_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn search(&self, filter: UserFilter, limit: i64) -> Result<Vec<User>> {
        let mut query = String::from(
            "SELECT * FROM users WHERE is_deleted = FALSE"
        );

        if let Some(org_id) = filter.organization_id {
            query.push_str(&format!(" AND organization_id = '{}'", org_id));
        }
        if let Some(user_type) = filter.user_type {
            query.push_str(&format!(" AND user_type = '{:?}'", user_type).to_uppercase());
        }
        if let Some(status) = filter.user_status {
            query.push_str(&format!(" AND user_status = '{:?}'", status).to_uppercase());
        }
        if let Some(dept) = filter.department {
            query.push_str(&format!(" AND department = '{}'", dept));
        }
        if let Some(search) = filter.search_query {
            query.push_str(&format!(
                " AND (first_name ILIKE '%{}%' OR last_name ILIKE '%{}%' OR email ILIKE '%{}%')",
                search, search, search
            ));
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", limit));

        let users = sqlx::query_as::<_, User>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(users)
    }

    pub async fn update(&self, input: UpdateUserInput) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                mobile_number = COALESCE($3, mobile_number),
                professional_title = COALESCE($4, professional_title),
                qualification = COALESCE($5, qualification),
                specialization = COALESCE($6, specialization),
                department = COALESCE($7, department),
                updated_at = NOW()
            WHERE id = $8 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&input.first_name)
        .bind(&input.last_name)
        .bind(&input.mobile_number)
        .bind(&input.professional_title)
        .bind(&input.qualification)
        .bind(&input.specialization)
        .bind(&input.department)
        .bind(input.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn update_password(&self, user_id: Uuid, password_hash: String) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                password_hash = $1,
                password_changed_at = NOW(),
                must_change_password = FALSE,
                updated_at = NOW()
            WHERE id = $2 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(&password_hash)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn update_status(&self, user_id: Uuid, status: UserStatus) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET user_status = $1, updated_at = NOW()
            WHERE id = $2 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(status)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn record_login_attempt(&self, user_id: Uuid, success: bool, ip_address: Option<String>) -> Result<()> {
        if success {
            sqlx::query(
                r#"
                UPDATE users
                SET
                    last_login_at = NOW(),
                    last_login_ip = $1::inet,
                    login_count = login_count + 1,
                    failed_login_attempts = 0
                WHERE id = $2
                "#
            )
            .bind(ip_address)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(Error::Database)?;
        } else {
            sqlx::query(
                r#"
                UPDATE users
                SET
                    failed_login_attempts = failed_login_attempts + 1,
                    last_failed_login_at = NOW(),
                    locked_until = CASE
                        WHEN failed_login_attempts + 1 >= 5 THEN NOW() + INTERVAL '30 minutes'
                        ELSE locked_until
                    END
                WHERE id = $1
                "#
            )
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(Error::Database)?;
        }

        Ok(())
    }

    pub async fn verify_email(&self, user_id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                email_verified = TRUE,
                email_verified_at = NOW(),
                email_verification_token = NULL,
                user_status = CASE
                    WHEN user_status = 'PENDING_VERIFICATION' THEN 'ACTIVE'::user_status
                    ELSE user_status
                END,
                updated_at = NOW()
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn set_password_reset_token(&self, user_id: Uuid, token: String, expires_at: DateTime<Utc>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET
                password_reset_token = $1,
                password_reset_expires_at = $2,
                updated_at = NOW()
            WHERE id = $3
            "#
        )
        .bind(&token)
        .bind(expires_at)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE password_reset_token = $1
              AND password_reset_expires_at > NOW()
              AND is_deleted = FALSE
            "#
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn clear_reset_token(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET
                password_reset_token = NULL,
                password_reset_expires_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    async fn generate_user_code(&self, user_type: &UserType) -> Result<String> {
        let org_code = "LAB"; // Should fetch from organization
        let type_str = match user_type {
            UserType::Doctor => "DOC",
            UserType::Technician => "TCH",
            UserType::Nurse => "NRS",
            _ => "USR",
        };

        let row = sqlx::query("SELECT nextval('user_sequence')")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        let sequence: i64 = row.try_get(0).map_err(|e| Error::Database(sqlx::Error::Decode(Box::new(e))))?;

        let base_id = format!("{}-{}-{:06}", org_code, type_str, sequence);
        let checksum = common::utils::calculate_luhn_check_digit(&base_id);

        Ok(format!("{}{}", base_id, checksum))
    }
}

// ============================================================================
// Role Repository
// ============================================================================

#[derive(Clone)]
pub struct RoleRepository {
    pool: PgPool,
}

impl RoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateRoleInput, user_id: Uuid) -> Result<Role> {
        input.validate()?;

        let role = sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO role (
                id, role_code, role_name, description,
                organization_id, is_system_role, created_by
            )
            VALUES ($1, $2, $3, $4, $5, FALSE, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&input.role_code)
        .bind(&input.role_name)
        .bind(&input.description)
        .bind(input.organization_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        // Assign permissions to role
        for permission_id in input.permission_ids {
            self.assign_permission_to_role(role.id, permission_id).await?;
        }

        Ok(role)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT * FROM role WHERE id = $1 AND is_active = TRUE"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(role)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT * FROM role WHERE role_code = $1 AND is_active = TRUE"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(role)
    }

    pub async fn get_all(&self, org_id: Option<Uuid>) -> Result<Vec<Role>> {
        let roles = if let Some(org) = org_id {
            sqlx::query_as::<_, Role>(
                r#"
                SELECT * FROM role
                WHERE (organization_id = $1 OR is_system_role = TRUE)
                  AND is_active = TRUE
                ORDER BY role_name
                "#
            )
            .bind(org)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Role>(
                "SELECT * FROM role WHERE is_active = TRUE ORDER BY role_name"
            )
            .fetch_all(&self.pool)
            .await
        }
        .map_err(Error::Database)?;

        Ok(roles)
    }

    pub async fn assign_permission_to_role(&self, role_id: Uuid, permission_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO role_permission (id, role_id, permission_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (role_id, permission_id) DO NOTHING
            "#
        )
        .bind(Uuid::new_v4())
        .bind(role_id)
        .bind(permission_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn get_role_permissions(&self, role_id: Uuid) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as::<_, Permission>(
            r#"
            SELECT p.* FROM permission p
            INNER JOIN role_permission rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1 AND p.is_active = TRUE
            ORDER BY p.module, p.action
            "#
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(permissions)
    }
}

// ============================================================================
// Permission Repository
// ============================================================================

#[derive(Clone)]
pub struct PermissionRepository {
    pool: PgPool,
}

impl PermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as::<_, Permission>(
            "SELECT * FROM permission WHERE is_active = TRUE ORDER BY module, action"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(permissions)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Permission>> {
        let permission = sqlx::query_as::<_, Permission>(
            "SELECT * FROM permission WHERE permission_code = $1 AND is_active = TRUE"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(permission)
    }

    pub async fn get_by_module(&self, module: &str) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as::<_, Permission>(
            "SELECT * FROM permission WHERE module = $1 AND is_active = TRUE ORDER BY action"
        )
        .bind(module)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(permissions)
    }
}

// ============================================================================
// User Role Repository
// ============================================================================

#[derive(Clone)]
pub struct UserRoleRepository {
    pool: PgPool,
}

impl UserRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn assign_role(&self, input: AssignRoleInput, assigned_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_role (id, user_id, role_id, assigned_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#
        )
        .bind(Uuid::new_v4())
        .bind(input.user_id)
        .bind(input.role_id)
        .bind(assigned_by)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM user_role WHERE user_id = $1 AND role_id = $2")
            .bind(user_id)
            .bind(role_id)
            .execute(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            r#"
            SELECT r.* FROM role r
            INNER JOIN user_role ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
              AND r.is_active = TRUE
              AND (ur.valid_until IS NULL OR ur.valid_until > NOW())
            ORDER BY r.role_name
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(roles)
    }

    pub async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as::<_, Permission>(
            r#"
            SELECT DISTINCT p.* FROM permission p
            INNER JOIN role_permission rp ON p.id = rp.permission_id
            INNER JOIN user_role ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1
              AND p.is_active = TRUE
              AND (ur.valid_until IS NULL OR ur.valid_until > NOW())
            ORDER BY p.module, p.action
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(permissions)
    }
}

// ============================================================================
// Session Repository
// ============================================================================

#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        session_token: String,
        access_token: String,
        refresh_token: String,
        device_info: (Option<String>, Option<String>, Option<String>),
        ip_address: Option<String>,
        expires_at: DateTime<Utc>,
        refresh_expires_at: DateTime<Utc>,
    ) -> Result<UserSession> {
        let session = sqlx::query_as::<_, UserSession>(
            r#"
            INSERT INTO user_session (
                id, session_token, user_id, device_id, device_name, device_type,
                ip_address, access_token, refresh_token, expires_at, refresh_token_expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7::inet, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&session_token)
        .bind(user_id)
        .bind(device_info.0)
        .bind(device_info.1)
        .bind(device_info.2)
        .bind(ip_address)
        .bind(&access_token)
        .bind(&refresh_token)
        .bind(expires_at)
        .bind(refresh_expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(session)
    }

    pub async fn find_by_token(&self, token: &str) -> Result<Option<UserSession>> {
        let session = sqlx::query_as::<_, UserSession>(
            "SELECT * FROM user_session WHERE session_token = $1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(session)
    }

    pub async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<UserSession>> {
        let session = sqlx::query_as::<_, UserSession>(
            "SELECT * FROM user_session WHERE refresh_token = $1 AND session_status = 'ACTIVE'"
        )
        .bind(refresh_token)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(session)
    }

    pub async fn update_activity(&self, session_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE user_session SET last_activity_at = NOW() WHERE id = $1"
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn logout(&self, session_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_session
            SET session_status = 'LOGGED_OUT', logged_out_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn revoke_user_sessions(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE user_session SET session_status = 'REVOKED' WHERE user_id = $1 AND session_status = 'ACTIVE'"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<UserSession>> {
        let sessions = sqlx::query_as::<_, UserSession>(
            r#"
            SELECT * FROM user_session
            WHERE user_id = $1 AND session_status = 'ACTIVE'
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(sessions)
    }

    pub async fn cleanup_expired(&self) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE user_session
            SET session_status = 'EXPIRED'
            WHERE session_status = 'ACTIVE' AND expires_at < NOW()
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(result.rows_affected())
    }
}

// ============================================================================
// Activity Log Repository
// ============================================================================

#[derive(Clone)]
pub struct ActivityLogRepository {
    pool: PgPool,
}

impl ActivityLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log(
        &self,
        user_id: Option<Uuid>,
        session_id: Option<Uuid>,
        action: &str,
        module: &str,
        description: Option<String>,
        ip_address: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO activity_log (
                id, user_id, session_id, action, module, description, ip_address
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7::inet)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(session_id)
        .bind(action)
        .bind(module)
        .bind(description)
        .bind(ip_address)
        .execute(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    pub async fn get_user_activity(&self, user_id: Uuid, limit: i64) -> Result<Vec<ActivityLog>> {
        let logs = sqlx::query_as::<_, ActivityLog>(
            r#"
            SELECT * FROM activity_log
            WHERE user_id = $1
            ORDER BY performed_at DESC
            LIMIT $2
            "#
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(logs)
    }
}
