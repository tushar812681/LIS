# LIS Modern - Documentation Index

Comprehensive documentation for the Cloud-Native Laboratory Information System.

## 📚 Documentation Structure

### Architecture Documentation

#### System Design
- **[High-Level Design (HLD)](architecture/01-high-level-design.md)** - System architecture overview, component interactions, deployment architecture
- **[Low-Level Design (LLD)](architecture/02-low-level-design.md)** - Detailed microservice designs, algorithms, data structures
- **[Technology Stack](architecture/03-technology-stack.md)** - Technology choices and justifications
- **[Security Architecture](architecture/04-security-architecture.md)** - Security patterns, authentication, authorization, encryption
- **[Scalability & Performance](architecture/05-scalability-performance.md)** - Scaling strategies, performance optimization
- **[Disaster Recovery](architecture/06-disaster-recovery.md)** - Backup, failover, business continuity

#### Architecture Decision Records (ADRs)
- [ADR-001: Microservices Architecture](architecture/adr/001-microservices-architecture.md)
- [ADR-002: Rust for Backend](architecture/adr/002-rust-backend.md)
- [ADR-003: GraphQL over REST](architecture/adr/003-graphql-api.md)
- [ADR-004: Event-Driven with Kafka](architecture/adr/004-event-driven-kafka.md)
- [ADR-005: PostgreSQL + MongoDB](architecture/adr/005-database-choices.md)
- [ADR-006: Offline-First Architecture](architecture/adr/006-offline-first.md)
- [ADR-007: Multi-Tenant Design](architecture/adr/007-multi-tenant.md)

### Database Documentation

- **[ER Diagrams](database/er-diagrams.md)** - Comprehensive entity-relationship diagrams
- **[Schema Design](database/schema-design.md)** - PostgreSQL and MongoDB schemas
- **[Migration Strategy](database/migrations.md)** - Database migration processes
- **[Indexing Strategy](database/indexing.md)** - Performance optimization through indexes
- **[Data Retention](database/data-retention.md)** - NABL-compliant data archival

### API Documentation

- **[GraphQL Schema](api/graphql-schema.md)** - Complete GraphQL API reference
- **[WebSocket Events](api/websocket-events.md)** - Real-time event specifications
- **[REST APIs](api/rest-apis.md)** - Legacy integration endpoints
- **[HL7/FHIR Integration](api/hl7-fhir-integration.md)** - Healthcare interoperability
- **[Authentication](api/authentication.md)** - JWT, OAuth2, MFA implementation
- **[Rate Limiting](api/rate-limiting.md)** - API throttling and quotas

### Workflow Documentation

- **[Patient Registration Flow](workflows/patient-registration.md)**
- **[Sample Collection Flow](workflows/sample-collection.md)**
- **[Test Processing Flow](workflows/test-processing.md)**
- **[Quality Control Flow](workflows/quality-control.md)**
- **[Result Verification Flow](workflows/result-verification.md)**
- **[Report Generation Flow](workflows/report-generation.md)**
- **[Billing Flow](workflows/billing.md)**
- **[NABL Compliance Flow](workflows/nabl-compliance.md)**

### User Flow Diagrams

- **[Lab Technician Flows](workflows/user-flows/lab-technician.md)**
- **[Pathologist Flows](workflows/user-flows/pathologist.md)**
- **[Front Desk Flows](workflows/user-flows/front-desk.md)**
- **[Lab Director Flows](workflows/user-flows/lab-director.md)**
- **[Patient Flows](workflows/user-flows/patient.md)**
- **[Admin Flows](workflows/user-flows/admin.md)**

### Module Documentation

#### Core Modules
1. **[Patient Management](modules/01-patient-management.md)** - Registration, demographics, consent, history
2. **[Sample Management](modules/02-sample-management.md)** - Collection, tracking, storage, disposal
3. **[Test Catalog](modules/03-test-catalog.md)** - Test definitions, protocols, pricing
4. **[Equipment Integration](modules/04-equipment-integration.md)** - HL7/ASTM, middleware, QC
5. **[Quality Control](modules/05-quality-control.md)** - IQC, EQC, Westgard rules
6. **[Result Management](modules/06-result-management.md)** - Entry, validation, auto-verification
7. **[Reporting Engine](modules/07-reporting.md)** - Generation, templates, multi-channel delivery
8. **[Billing & RCM](modules/08-billing.md)** - Invoicing, payments, insurance claims
9. **[NABL Compliance](modules/09-nabl-compliance.md)** - Document control, audit trails, QMS
10. **[Analytics & BI](modules/10-analytics.md)** - Dashboards, KPIs, predictive models
11. **[Inventory Management](modules/11-inventory.md)** - Stock, reagents, procurement
12. **[Administration](modules/12-administration.md)** - Users, roles, configuration

#### India-Specific Features
- **[WhatsApp Integration](modules/india/whatsapp-integration.md)**
- **[Multi-Language Support](modules/india/multi-language.md)**
- **[Offline-First Sync](modules/india/offline-sync.md)**
- **[Payment Gateway Integration](modules/india/payment-integration.md)**
- **[GST & E-Invoice](modules/india/gst-einvoice.md)**
- **[ABDM Integration](modules/india/abdm-integration.md)**
- **[Aadhaar Verification](modules/india/aadhaar-verification.md)**

### AI/ML Documentation

- **[Auto-Verification Engine](ml/auto-verification.md)** - Rule-based + ML models
- **[Predictive TAT](ml/predictive-tat.md)** - Turnaround time optimization
- **[QC Anomaly Detection](ml/qc-anomaly-detection.md)** - Quality control intelligence
- **[Equipment Maintenance Prediction](ml/equipment-prediction.md)** - Predictive maintenance
- **[Model Training Pipeline](ml/training-pipeline.md)** - MLOps processes
- **[Model Versioning](ml/model-versioning.md)** - MLflow integration

### Deployment Documentation

- **[Docker Setup](deployment/docker-setup.md)** - Containerization guide
- **[Kubernetes Deployment](deployment/kubernetes.md)** - K8s manifests and configuration
- **[CI/CD Pipeline](deployment/cicd-pipeline.md)** - GitHub Actions workflows
- **[Infrastructure as Code](deployment/infrastructure-as-code.md)** - Terraform configurations
- **[Monitoring & Observability](deployment/monitoring.md)** - Prometheus, Grafana, Jaeger
- **[Cloud Provider Setup](deployment/cloud-providers.md)** - AWS, Azure, GCP
- **[Multi-Region Deployment](deployment/multi-region.md)** - Geographic distribution

### Security & Compliance

- **[NABL Compliance Guide](compliance/nabl-compliance.md)** - ISO 15189:2022
- **[DPDP 2023 Compliance](compliance/dpdp-compliance.md)** - Indian data protection
- **[HIPAA Readiness](compliance/hipaa-compliance.md)** - US market preparation
- **[Security Best Practices](compliance/security-practices.md)** - Secure coding guidelines
- **[Audit Trail Requirements](compliance/audit-trails.md)** - Regulatory compliance
- **[Data Localization](compliance/data-localization.md)** - Geographic data requirements
- **[Penetration Testing](compliance/penetration-testing.md)** - Security testing procedures

### User Guides

#### For Lab Staff
- **[Quick Start Guide](user-guides/quick-start.md)**
- **[Patient Registration Guide](user-guides/patient-registration.md)**
- **[Sample Collection Guide](user-guides/sample-collection.md)**
- **[Result Entry Guide](user-guides/result-entry.md)**
- **[Report Generation Guide](user-guides/report-generation.md)**
- **[Quality Control Guide](user-guides/quality-control.md)**

#### For Administrators
- **[System Configuration](user-guides/admin/system-configuration.md)**
- **[User Management](user-guides/admin/user-management.md)**
- **[Test Catalog Setup](user-guides/admin/test-catalog-setup.md)**
- **[Equipment Configuration](user-guides/admin/equipment-configuration.md)**
- **[Billing Setup](user-guides/admin/billing-setup.md)**
- **[Reports Configuration](user-guides/admin/reports-configuration.md)**

#### For Lab Directors
- **[Dashboard Overview](user-guides/director/dashboard.md)**
- **[Analytics & Reports](user-guides/director/analytics.md)**
- **[NABL Compliance](user-guides/director/nabl-compliance.md)**
- **[Quality Metrics](user-guides/director/quality-metrics.md)**
- **[Business Intelligence](user-guides/director/business-intelligence.md)**

### Development Guides

- **[Development Setup](development/setup.md)** - Local development environment
- **[Backend Development](development/backend-development.md)** - Rust development guide
- **[Frontend Development](development/frontend-development.md)** - Next.js development guide
- **[Testing Guide](development/testing.md)** - Unit, integration, E2E tests
- **[Code Style Guide](development/code-style.md)** - Coding standards
- **[Debugging Guide](development/debugging.md)** - Troubleshooting techniques

### API References

- **[GraphQL API Reference](api-reference/graphql.md)** - Auto-generated from schema
- **[WebSocket API Reference](api-reference/websockets.md)** - Event specifications
- **[REST API Reference](api-reference/rest.md)** - OpenAPI/Swagger documentation
- **[SDK Documentation](api-reference/sdk.md)** - Client libraries

### Troubleshooting

- **[Common Issues](troubleshooting/common-issues.md)**
- **[Performance Debugging](troubleshooting/performance.md)**
- **[Database Issues](troubleshooting/database.md)**
- **[Network Issues](troubleshooting/network.md)**
- **[Integration Issues](troubleshooting/integration.md)**

## 🚀 Quick Links

### For New Users
1. Start with [Quick Start Guide](user-guides/quick-start.md)
2. Review [Patient Registration Guide](user-guides/patient-registration.md)
3. Explore role-specific guides

### For Developers
1. Setup [Development Environment](development/setup.md)
2. Review [High-Level Design](architecture/01-high-level-design.md)
3. Check [API Documentation](api/graphql-schema.md)
4. Read [Contributing Guidelines](../CONTRIBUTING.md)

### For System Administrators
1. Review [Kubernetes Deployment](deployment/kubernetes.md)
2. Setup [Monitoring](deployment/monitoring.md)
3. Configure [Security](compliance/security-practices.md)
4. Plan [Disaster Recovery](architecture/06-disaster-recovery.md)

### For Lab Directors
1. Review [Dashboard Overview](user-guides/director/dashboard.md)
2. Explore [Analytics Capabilities](user-guides/director/analytics.md)
3. Understand [NABL Compliance](user-guides/director/nabl-compliance.md)

## 📝 Documentation Standards

### Markdown Guidelines
- Use clear, descriptive headings
- Include table of contents for long documents
- Add code examples where applicable
- Use diagrams (Mermaid) for visual clarity
- Keep sentences concise and clear

### Diagram Standards
- Use Mermaid for flowcharts and diagrams
- Include alternative text descriptions
- Keep diagrams focused and readable
- Update diagrams when architecture changes

### Code Examples
- Provide complete, working examples
- Include comments explaining complex logic
- Show both success and error cases
- Use realistic data in examples

## 🔄 Document Updates

### Versioning
- Documentation is versioned with software releases
- Major version changes documented in CHANGELOG
- Breaking changes clearly highlighted

### Maintenance
- Review quarterly for accuracy
- Update with each major release
- Archive old versions
- Keep links up-to-date

## 📞 Documentation Feedback

Found an error or have suggestions?

- **GitHub Issues**: [Report documentation issues](https://github.com/your-org/lis-modern/issues)
- **Email**: docs@lis-modern.com
- **Discussions**: [GitHub Discussions](https://github.com/your-org/lis-modern/discussions)

## 📜 License

Documentation is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

---

**Last Updated**: 2024-11-05
**Version**: 1.0.0
