# Production Security Hardening Guide

**Version**: 1.0.0
**Date**: 2025-11-15
**Status**: Production-Ready Checklist

---

## Overview

This guide provides comprehensive security hardening steps for deploying the LIS backend to production. Follow all steps to ensure HIPAA, DPDP 2023, and NABL compliance.

---

## 1. Authentication & Authorization

### 1.1 JWT Configuration ✓ CRITICAL

```bash
# Generate strong JWT secret (NEVER use default)
openssl rand -base64 64

# Update .env
JWT_SECRET=<generated-64-char-secret>
JWT_EXPIRATION_HOURS=8  # Reduce from 24 to 8 hours
JWT_REFRESH_EXPIRATION_DAYS=7
```

**Checklist**:
- [ ] JWT secret is randomly generated (64+ characters)
- [ ] JWT expiration is set to 8 hours or less
- [ ] Refresh tokens implemented with rotation
- [ ] JWT tokens include user roles and permissions
- [ ] Blacklist mechanism for revoked tokens

### 1.2 Password Policy

**Implementation** (in user-service):
```rust
// Minimum password requirements
- Length: 12+ characters
- Uppercase: 1+ characters
- Lowercase: 1+ characters
- Numbers: 1+ characters
- Special characters: 1+ characters
- No common passwords (check against list)
- Password history: last 5 passwords
- Expiration: 90 days
```

**Checklist**:
- [ ] Argon2 password hashing implemented
- [ ] Password complexity enforced
- [ ] Password history tracked (last 5)
- [ ] Account lockout after 5 failed attempts
- [ ] Temporary lockout: 30 minutes
- [ ] Password reset requires email verification

### 1.3 Multi-Factor Authentication (MFA)

**Implementation Status**: ⚠️ NOT IMPLEMENTED

**Required**:
- [ ] TOTP (Time-based OTP) support
- [ ] SMS OTP as fallback
- [ ] Recovery codes (10 one-time codes)
- [ ] MFA enforcement for admin users
- [ ] MFA optional for regular users

---

## 2. Data Encryption

### 2.1 Database Encryption at Rest

**PostgreSQL Encryption**:
```bash
# Enable transparent data encryption (TDE)
# AWS RDS: Enable encryption at cluster creation
# Self-hosted: Use dm-crypt/LUKS

# Encrypt specific columns (PII data)
CREATE EXTENSION IF NOT EXISTS pgcrypto;

# Example: Encrypt Aadhaar numbers
ALTER TABLE patients ADD COLUMN aadhaar_encrypted BYTEA;

UPDATE patients SET aadhaar_encrypted = pgp_sym_encrypt(aadhaar, 'encryption-key');
```

**Checklist**:
- [ ] Database encryption at rest enabled
- [ ] PII fields encrypted (Aadhaar, mobile, email)
- [ ] Encryption keys stored in AWS KMS / HashiCorp Vault
- [ ] Key rotation policy implemented (quarterly)

### 2.2 Transport Layer Security (TLS)

**HTTPS Configuration**:
```nginx
# Nginx configuration
server {
    listen 443 ssl http2;
    server_name api.yourlabdomain.com;

    ssl_certificate /etc/letsencrypt/live/api.yourlabdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourlabdomain.com/privkey.pem;

    ssl_protocols TLSv1.3 TLSv1.2;
    ssl_ciphers 'ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256';
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;

    # HSTS
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
}
```

**Checklist**:
- [ ] TLS 1.3 configured (minimum TLS 1.2)
- [ ] Valid SSL certificate (Let's Encrypt or commercial)
- [ ] HSTS header enabled
- [ ] All HTTP traffic redirected to HTTPS
- [ ] mTLS for service-to-service communication

### 2.3 Service-to-Service Encryption (mTLS)

**Istio Service Mesh** (Recommended):
```yaml
# Enable mTLS for all services
apiVersion: security.istio.io/v1beta1
kind: PeerAuthentication
metadata:
  name: default
  namespace: lis
spec:
  mtls:
    mode: STRICT
```

**Checklist**:
- [ ] mTLS enabled for all inter-service communication
- [ ] Service mesh deployed (Istio/Linkerd)
- [ ] Certificate rotation automated
- [ ] Certificate validity: 90 days

---

## 3. Network Security

### 3.1 Firewall Rules

**AWS Security Groups**:
```
Inbound Rules:
- Port 443 (HTTPS): 0.0.0.0/0 (public)
- Port 22 (SSH): Your-IP-Only
- Port 5432 (PostgreSQL): VPC-Only
- Port 6379 (Redis): VPC-Only
- Port 9092 (Kafka): VPC-Only

Outbound Rules:
- Port 443 (HTTPS): 0.0.0.0/0 (for external APIs)
- Port 25/587 (SMTP): 0.0.0.0/0 (for email)
```

**Checklist**:
- [ ] All database ports restricted to VPC only
- [ ] SSH access restricted to specific IPs
- [ ] No direct internet access to internal services
- [ ] All outbound traffic logged

### 3.2 API Rate Limiting

**Implementation** (in API Gateway):
```rust
// Rate limiting per user
- 100 requests/minute per user
- 10,000 requests/hour per organization
- 1,000 GraphQL queries/minute (complexity-based)

// Rate limiting per IP
- 1,000 requests/hour per IP (anonymous)
- Burst: 20 requests/second
```

**Checklist**:
- [ ] Per-user rate limiting implemented
- [ ] Per-IP rate limiting for anonymous users
- [ ] GraphQL query complexity limits (max 1000)
- [ ] Rate limit headers returned (X-RateLimit-*)
- [ ] Redis-backed rate limiter

### 3.3 DDoS Protection

**Cloudflare / AWS Shield**:
```
- Layer 7 DDoS protection
- WAF rules for common attacks
- Rate limiting at edge
- Geographic restrictions (if needed)
```

**Checklist**:
- [ ] DDoS protection service enabled (Cloudflare/AWS Shield)
- [ ] WAF rules configured
- [ ] Bot detection enabled
- [ ] Challenge page for suspicious traffic

---

## 4. Input Validation & Sanitization

### 4.1 SQL Injection Prevention

**Current Status**: ✓ PROTECTED (sqlx with parameterized queries)

```rust
// GOOD: Parameterized query
sqlx::query_as!(Patient, "SELECT * FROM patients WHERE id = $1", id)

// BAD: String concatenation (NEVER do this)
format!("SELECT * FROM patients WHERE id = {}", id)
```

**Checklist**:
- [ ] All database queries use parameterized statements
- [ ] No raw SQL string concatenation
- [ ] Input validation on all user inputs
- [ ] ORM/query builder used (sqlx)

### 4.2 GraphQL Query Validation

**Implementation**:
```rust
// Query depth limiting
max_depth: 10

// Query complexity limiting
max_complexity: 1000

// Query timeout
timeout: 30 seconds

// Disable introspection in production
introspection: false
```

**Checklist**:
- [ ] Query depth limited to 10
- [ ] Query complexity scoring implemented
- [ ] Introspection disabled in production
- [ ] Query timeout enforced (30 seconds)
- [ ] Malicious query patterns blocked

### 4.3 File Upload Validation

**Implementation** (in file-service):
```rust
// File type validation
allowed_types: ["pdf", "jpg", "jpeg", "png", "doc", "docx"]

// File size validation
max_size: 50 MB

// Virus scanning
clamav_enabled: true

// Content type verification
verify_mime_type: true
```

**Checklist**:
- [ ] File type whitelist enforced
- [ ] File size limits enforced (50MB)
- [ ] MIME type verification
- [ ] Virus scanning (ClamAV integration)
- [ ] File content inspection (magic bytes)
- [ ] Uploaded files stored outside web root

---

## 5. Logging & Monitoring

### 5.1 Audit Logging

**Implementation** (all services):
```rust
// Log all security events
- User login/logout
- Failed login attempts
- Password changes
- Permission changes
- Data access (PII)
- Data modifications
- Configuration changes
- File downloads
```

**Checklist**:
- [ ] All authentication events logged
- [ ] All authorization failures logged
- [ ] All data modifications logged
- [ ] Logs are immutable (append-only)
- [ ] Logs retained for 7+ years (NABL requirement)
- [ ] Logs include: timestamp, user, IP, action, resource

### 5.2 Security Monitoring

**SIEM Integration**:
```
- Centralized log collection (ELK/Splunk)
- Real-time alerting for:
  * Multiple failed login attempts
  * Unusual data access patterns
  * Privilege escalation attempts
  * Suspicious API usage
  * Database queries with unusual patterns
```

**Checklist**:
- [ ] Centralized logging configured
- [ ] Real-time alerts configured
- [ ] Security dashboards created
- [ ] Anomaly detection enabled
- [ ] Incident response plan documented

### 5.3 Prometheus Alerts

**Alert Rules** (`alerts.yml`):
```yaml
groups:
  - name: security
    rules:
      - alert: HighFailedLoginRate
        expr: rate(failed_login_total[5m]) > 10
        annotations:
          summary: "High failed login rate detected"

      - alert: UnauthorizedAccessAttempt
        expr: increase(unauthorized_access_total[5m]) > 5
        annotations:
          summary: "Multiple unauthorized access attempts"

      - alert: DatabaseConnectionSpiking
        expr: pg_connections_active > 90
        annotations:
          summary: "Unusual database connection spike"
```

**Checklist**:
- [ ] Prometheus alert rules configured
- [ ] AlertManager configured for notifications
- [ ] On-call rotation defined
- [ ] Runbooks created for each alert

---

## 6. DPDP 2023 Compliance (India)

### 6.1 Consent Management

**Implementation Required**:
```rust
// Consent model
struct Consent {
    id: UUID,
    user_id: UUID,
    purpose: ConsentPurpose,  // e.g., "lab_test_processing"
    granted_at: DateTime,
    expires_at: Option<DateTime>,
    revoked_at: Option<DateTime>,
    version: i32,
}

enum ConsentPurpose {
    LabTestProcessing,
    DataSharing,
    Marketing,
    Research,
}
```

**Checklist**:
- [ ] Explicit consent obtained for data processing
- [ ] Consent can be withdrawn anytime
- [ ] Purpose limitation enforced
- [ ] Consent records maintained (audit trail)
- [ ] Data minimization principle followed

### 6.2 Right to Erasure

**Implementation**:
```rust
// Anonymization function
async fn anonymize_patient_data(patient_id: UUID) -> Result<()> {
    // Replace PII with anonymized data
    update_patient(patient_id, {
        name: "ANONYMIZED",
        email: "deleted@anonymized.com",
        mobile: "XXXXX-XXXXX",
        aadhaar: "XXXX-XXXX-XXXX",
        address: "ANONYMIZED",
        // Keep non-PII for analytics
        age_at_deletion: patient.age,
        gender: patient.gender,
    });
}
```

**Checklist**:
- [ ] Data deletion/anonymization API implemented
- [ ] User can request data deletion
- [ ] Deletion processed within 30 days
- [ ] Anonymization preserves analytics value
- [ ] Audit trail of deletion requests

### 6.3 Data Breach Notification

**Process**:
```
1. Detection: Automated monitoring detects breach
2. Assessment: Security team assesses impact (within 1 hour)
3. Notification: Affected users notified (within 72 hours)
4. Reporting: CERT-In notified (within 6 hours)
5. Remediation: Vulnerability patched
6. Documentation: Incident documented
```

**Checklist**:
- [ ] Breach detection system in place
- [ ] Incident response plan documented
- [ ] CERT-In notification process defined
- [ ] User notification templates prepared
- [ ] Post-mortem process defined

---

## 7. NABL ISO 15189:2022 Compliance

### 7.1 Document Control

**Implementation**:
```rust
// Document version control
struct Document {
    id: UUID,
    title: String,
    version: i32,
    status: DocumentStatus,  // Draft, UnderReview, Approved, Published, Obsolete
    approved_by: Option<UUID>,
    approved_at: Option<DateTime>,
    effective_date: Option<DateTime>,
    review_date: DateTime,  // Annual review
    hash: String,  // SHA-256 for integrity
}
```

**Checklist**:
- [ ] All SOPs versioned and controlled
- [ ] Approval workflow implemented
- [ ] Annual review process defined
- [ ] Obsolete documents marked clearly
- [ ] Document integrity verified (hashing)

### 7.2 Quality Control Data Retention

**Retention Policy**:
```
- Patient data: 7 years minimum (never delete test results)
- QC data: 10 years
- Audit logs: 7 years
- Calibration records: 7 years
- Training records: Permanent
```

**Checklist**:
- [ ] Retention policies configured in database
- [ ] Automated archival to cold storage
- [ ] Backup and restore tested quarterly
- [ ] Data integrity checks implemented

---

## 8. Secrets Management

### 8.1 HashiCorp Vault (Recommended)

**Setup**:
```bash
# Deploy Vault
docker run -d --name=vault \
  -p 8200:8200 \
  vault server -dev

# Store secrets
vault kv put secret/lis/database password=supersecret
vault kv put secret/lis/jwt secret=jwt-secret-here
vault kv put secret/lis/razorpay key_id=rzp_key key_secret=rzp_secret

# Application retrieves secrets at runtime
```

**Checklist**:
- [ ] Vault deployed and configured
- [ ] All secrets stored in Vault
- [ ] No secrets in .env file (production)
- [ ] Secret rotation policy defined
- [ ] Vault audit logging enabled

### 8.2 AWS Secrets Manager (Alternative)

```bash
# Store secret
aws secretsmanager create-secret \
  --name lis/database/password \
  --secret-string "supersecret"

# Retrieve in application
secret=$(aws secretsmanager get-secret-value \
  --secret-id lis/database/password \
  --query SecretString --output text)
```

**Checklist**:
- [ ] Secrets Manager configured
- [ ] Automatic secret rotation enabled
- [ ] IAM roles configured for service access
- [ ] Secrets encrypted with KMS

---

## 9. Backup & Disaster Recovery

### 9.1 Automated Backups

**Configuration**:
```bash
# Database backups (every 4 hours)
0 */4 * * * /scripts/backup-databases.sh

# Retention policy
- Hourly: Keep last 24
- Daily: Keep last 30
- Weekly: Keep last 12
- Monthly: Keep last 12
- Yearly: Keep last 7
```

**Checklist**:
- [ ] Automated backups configured (every 4 hours)
- [ ] Backups stored in separate region
- [ ] Backup encryption enabled
- [ ] Backup restore tested monthly
- [ ] Point-in-time recovery enabled

### 9.2 Disaster Recovery Plan

**RTO/RPO Targets**:
```
- Recovery Time Objective (RTO): <1 hour
- Recovery Point Objective (RPO): <15 minutes
- Multi-region setup: Mumbai (primary) + Delhi (DR)
```

**Checklist**:
- [ ] DR environment in separate region
- [ ] Database replication configured
- [ ] Failover process documented
- [ ] DR drills conducted quarterly
- [ ] Runbook for failover procedure

---

## 10. Container Security

### 10.1 Docker Image Hardening

**Best Practices**:
```dockerfile
# Use minimal base images
FROM debian:bookworm-slim

# Run as non-root user
RUN useradd -m -u 1000 appuser
USER appuser

# Scan for vulnerabilities
RUN trivy image myimage:latest

# No secrets in images
# Use .dockerignore
```

**Checklist**:
- [ ] Minimal base images used
- [ ] Containers run as non-root user
- [ ] No secrets in Docker images
- [ ] Images scanned for vulnerabilities (Trivy)
- [ ] Image signing implemented (Docker Content Trust)
- [ ] Regular image updates

### 10.2 Kubernetes Security

**Pod Security Standards**:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: secure-pod
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    fsGroup: 1000
  containers:
  - name: app
    securityContext:
      allowPrivilegeEscalation: false
      capabilities:
        drop: ["ALL"]
      readOnlyRootFilesystem: true
```

**Checklist**:
- [ ] Pod Security Policies configured
- [ ] RBAC configured (least privilege)
- [ ] Network Policies implemented
- [ ] Secrets encrypted at rest (etcd encryption)
- [ ] Kubernetes audit logging enabled

---

## 11. Third-Party Security

### 11.1 Dependency Scanning

**Tools**:
```bash
# Rust: cargo-audit
cargo install cargo-audit
cargo audit

# Docker: Trivy
trivy image lis-patient-service:latest

# GitHub: Dependabot
# Enable in repository settings
```

**Checklist**:
- [ ] Automated dependency scanning enabled
- [ ] Critical vulnerabilities patched within 24 hours
- [ ] High vulnerabilities patched within 7 days
- [ ] Dependency update policy defined
- [ ] Security advisories monitored

### 11.2 API Security (WhatsApp, Razorpay, GSTN)

**Checklist**:
- [ ] API keys stored in Vault/Secrets Manager
- [ ] Webhook signatures verified
- [ ] API rate limits respected
- [ ] API timeouts configured (30 seconds)
- [ ] Circuit breakers implemented
- [ ] API credentials rotated quarterly

---

## 12. Incident Response

### 12.1 Security Incident Response Plan

**Phases**:
```
1. Preparation
   - Incident response team defined
   - Tools and playbooks ready

2. Detection & Analysis
   - Automated monitoring detects incident
   - Severity assessed (Critical, High, Medium, Low)

3. Containment
   - Isolate affected systems
   - Block malicious IPs/users

4. Eradication
   - Remove malware/vulnerability
   - Patch systems

5. Recovery
   - Restore from clean backups
   - Verify system integrity

6. Post-Incident
   - Root cause analysis
   - Update security controls
   - Report to management/regulators
```

**Checklist**:
- [ ] Incident response team designated
- [ ] Escalation matrix defined
- [ ] Playbooks for common incidents
- [ ] Communication templates prepared
- [ ] Post-mortem process defined

---

## 13. Compliance Checklist

### HIPAA (for US expansion)
- [ ] Administrative safeguards implemented
- [ ] Physical safeguards implemented
- [ ] Technical safeguards implemented
- [ ] Business Associate Agreements (BAA) signed
- [ ] Risk analysis conducted annually

### DPDP 2023 (India)
- [ ] Privacy policy published
- [ ] Consent management implemented
- [ ] Data Principal rights implemented
- [ ] Data breach notification process
- [ ] Data Protection Officer appointed

### NABL ISO 15189:2022
- [ ] Quality management system documented
- [ ] Document control implemented
- [ ] Training records maintained
- [ ] Equipment calibration tracked
- [ ] Proficiency testing records maintained
- [ ] Audit trails for all QC data

### SOC 2 Type II (planned)
- [ ] Security policies documented
- [ ] Access controls implemented
- [ ] Change management process
- [ ] Incident response process
- [ ] Annual penetration testing

---

## 14. Security Testing

### 14.1 Automated Security Scanning

**Tools**:
```bash
# OWASP ZAP (API scanning)
docker run -t owasp/zap2docker-stable zap-baseline.py \
  -t https://api.yourlabdomain.com

# SonarQube (code quality & security)
sonar-scanner \
  -Dsonar.projectKey=lis-backend \
  -Dsonar.sources=. \
  -Dsonar.host.url=http://localhost:9000

# Bandit (Python - for ML service)
bandit -r ./ml-service/
```

**Checklist**:
- [ ] Automated security scanning in CI/CD
- [ ] SAST (Static Application Security Testing)
- [ ] DAST (Dynamic Application Security Testing)
- [ ] Dependency vulnerability scanning
- [ ] Container image scanning

### 14.2 Manual Security Testing

**Schedule**:
```
- Quarterly: Internal security audit
- Annually: External penetration test
- Annually: Code security review
- As needed: Threat modeling
```

**Checklist**:
- [ ] Penetration testing conducted annually
- [ ] Bug bounty program considered
- [ ] Security code review for critical features
- [ ] Threat modeling for new features

---

## 15. Production Deployment Checklist

**Pre-Deployment**:
- [ ] All secrets rotated and stored in Vault
- [ ] TLS certificates configured
- [ ] Firewall rules configured
- [ ] Rate limiting enabled
- [ ] DDoS protection enabled
- [ ] Backup system tested
- [ ] Monitoring and alerting configured
- [ ] Security scanning passed
- [ ] Penetration test completed
- [ ] Incident response plan ready

**Post-Deployment**:
- [ ] All health checks passing
- [ ] Logs flowing to centralized logging
- [ ] Metrics visible in Grafana
- [ ] Alerts triggered correctly
- [ ] Backup job running successfully
- [ ] SSL certificate valid
- [ ] No secrets exposed in logs
- [ ] Security headers present

---

## Summary

Following this security hardening guide will ensure:
- ✅ DPDP 2023 compliance (India)
- ✅ NABL ISO 15189:2022 compliance
- ✅ HIPAA-ready (for international expansion)
- ✅ SOC 2 Type II preparation
- ✅ Production-grade security posture

**Next Steps**:
1. Review this checklist with security team
2. Implement critical items (marked CRITICAL)
3. Schedule quarterly security audits
4. Conduct annual penetration testing
5. Train staff on security best practices

---

**Document Owner**: Security Team
**Last Reviewed**: 2025-11-15
**Next Review**: 2026-02-15 (Quarterly)

