use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub organization_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        organization_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
        expiration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(expiration_hours)).timestamp();

        Self {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp(),
            organization_id: organization_id.to_string(),
            roles,
            permissions,
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| Error::InvalidToken)
    }

    pub fn organization_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.organization_id)
            .map_err(|_| Error::InvalidToken)
    }
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(&self, claims: Claims) -> Result<String> {
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| Error::InternalServerError)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .map_err(|e| {
            if e.to_string().contains("expired") {
                Error::TokenExpired
            } else {
                Error::InvalidToken
            }
        })?;

        Ok(token_data.claims)
    }
}

// Password hashing utilities
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| Error::InternalServerError)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| Error::InternalServerError)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "SecurePassword123!";
        let hash = PasswordService::hash_password(password).unwrap();

        assert!(PasswordService::verify_password(password, &hash).unwrap());
        assert!(!PasswordService::verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token() {
        let jwt_service = JwtService::new("test_secret_key");

        let claims = Claims::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            vec!["TECHNICIAN".to_string()],
            vec!["READ_SAMPLES".to_string()],
            24,
        );

        let token = jwt_service.generate_token(claims.clone()).unwrap();
        let verified_claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, verified_claims.sub);
        assert!(verified_claims.has_permission("READ_SAMPLES"));
    }
}
