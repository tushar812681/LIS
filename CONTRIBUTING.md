# Contributing to LIS Modern

Thank you for your interest in contributing to LIS Modern! This document provides guidelines and information for contributors.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Documentation](#documentation)
7. [Pull Request Process](#pull-request-process)
8. [Security](#security)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. We pledge to:

- Be respectful and inclusive
- Welcome diverse perspectives
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment, discrimination, or hate speech
- Trolling or insulting comments
- Public or private harassment
- Publishing others' private information
- Any conduct that could reasonably be considered inappropriate

## Getting Started

### Prerequisites

**Backend Development (Rust):**
- Rust 1.75 or later
- cargo, rustc, rustfmt, clippy
- PostgreSQL 16+
- MongoDB 7+
- Redis 7+
- Kafka 3.6+

**Frontend Development (Next.js):**
- Node.js 20+
- npm or yarn or pnpm
- TypeScript knowledge

**Infrastructure:**
- Docker & Docker Compose
- Kubernetes (minikube for local)
- kubectl

### Local Development Setup

1. **Clone the repository:**
```bash
git clone https://github.com/your-org/lis-modern.git
cd lis-modern
```

2. **Start infrastructure services:**
```bash
docker-compose up -d
```

3. **Setup backend:**
```bash
cd backend
cargo build
cargo test
cargo run
```

4. **Setup frontend:**
```bash
cd frontend
npm install
npm run dev
```

5. **Access services:**
- Frontend: http://localhost:3000
- Backend API: http://localhost:8000
- GraphQL Playground: http://localhost:8000/graphql
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3001
- Jaeger: http://localhost:16686

## Development Workflow

### Branch Naming Convention

```
<type>/<ticket-id>-<short-description>

Examples:
- feature/LIS-123-patient-registration
- bugfix/LIS-456-sample-tracking-error
- docs/LIS-789-api-documentation
- refactor/LIS-321-database-optimization
```

### Types

- `feature/` - New feature or enhancement
- `bugfix/` - Bug fixes
- `hotfix/` - Critical production fixes
- `docs/` - Documentation only changes
- `refactor/` - Code refactoring without functionality change
- `test/` - Adding or updating tests
- `chore/` - Build process, dependencies, tooling

### Commit Message Format

Follow the Conventional Commits specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Example:**
```
feat(patient): add Aadhaar verification integration

- Integrate UIDAI API for Aadhaar verification
- Add OTP-based authentication flow
- Implement consent management UI
- Add unit tests for verification service

Closes #123
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

## Coding Standards

### Rust Backend

**Style Guide:**
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Maximum line length: 100 characters

**Code Quality:**
```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run tests
cargo test

# Check code without building
cargo check
```

**Best Practices:**
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Handle errors explicitly (avoid `unwrap()` in production code)
- Use type system for safety
- Write unit tests for all business logic
- Use async/await for I/O operations

**Example:**
```rust
/// Validates patient demographics data
///
/// # Arguments
/// * `patient_data` - The patient information to validate
///
/// # Returns
/// * `Result<ValidatedPatient, ValidationError>`
///
/// # Examples
/// ```
/// let patient = PatientData { name: "John Doe", ... };
/// let validated = validate_patient_data(patient)?;
/// ```
pub async fn validate_patient_data(
    patient_data: PatientData,
) -> Result<ValidatedPatient, ValidationError> {
    // Implementation
}
```

### TypeScript/Next.js Frontend

**Style Guide:**
- Follow [Airbnb TypeScript Style Guide](https://github.com/airbnb/javascript)
- Use ESLint and Prettier
- Maximum line length: 100 characters
- Use named exports over default exports

**Code Quality:**
```bash
# Lint code
npm run lint

# Format code
npm run format

# Type check
npm run type-check

# Run tests
npm run test
```

**Best Practices:**
- Use TypeScript strict mode
- Prefer functional components with hooks
- Use React Server Components where applicable
- Implement proper error boundaries
- Write comprehensive unit tests
- Use semantic HTML
- Ensure accessibility (ARIA labels, keyboard navigation)

**Example:**
```typescript
/**
 * Patient Registration Form Component
 *
 * Handles patient demographic data entry with validation
 * and Aadhaar verification integration.
 */
interface PatientRegistrationProps {
  onSuccess: (patient: Patient) => void;
  onCancel: () => void;
}

export function PatientRegistration({
  onSuccess,
  onCancel,
}: PatientRegistrationProps) {
  // Implementation
}
```

### Database

**PostgreSQL:**
- Use migrations for schema changes
- Never modify migrations after they're merged
- Add appropriate indexes
- Use meaningful constraint names
- Document complex queries

**MongoDB:**
- Define schemas using validation
- Use appropriate indexes
- Document data models
- Use transactions for multi-document operations

## Testing Guidelines

### Test Coverage Requirements

- **Backend**: Minimum 80% code coverage
- **Frontend**: Minimum 70% code coverage
- **Critical paths**: 100% coverage required

### Test Types

**Unit Tests:**
```rust
// Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patient_validation() {
        let patient = create_test_patient();
        assert!(validate_patient(&patient).is_ok());
    }
}
```

```typescript
// TypeScript
describe('PatientService', () => {
  it('should validate patient data correctly', () => {
    const patient = createTestPatient();
    expect(validatePatient(patient)).toBe(true);
  });
});
```

**Integration Tests:**
- Test service interactions
- Test database operations
- Test API endpoints

**E2E Tests:**
- Test complete user workflows
- Use Playwright or Cypress
- Cover critical user journeys

### Running Tests

```bash
# Backend
cd backend
cargo test                    # All tests
cargo test --lib              # Library tests only
cargo test --test integration # Integration tests only

# Frontend
cd frontend
npm run test                  # Unit tests
npm run test:integration      # Integration tests
npm run test:e2e              # E2E tests
npm run test:coverage         # Coverage report
```

## Documentation

### Code Documentation

**Rust:**
- Add doc comments (`///`) for all public items
- Include examples in doc comments
- Run `cargo doc --open` to preview

**TypeScript:**
- Use JSDoc comments for functions and components
- Document complex logic
- Keep comments up-to-date

### Architecture Documentation

When making significant architectural changes:

1. Update relevant ADRs (Architecture Decision Records)
2. Update system diagrams
3. Update API documentation
4. Update user guides if applicable

### API Documentation

- GraphQL schemas are self-documenting
- Add descriptions to all types, fields, and arguments
- Provide examples for complex operations
- Update API changelog

## Pull Request Process

### Before Submitting

1. **Ensure code quality:**
   - All tests pass
   - Code is formatted
   - No linting errors
   - Documentation is updated

2. **Self-review:**
   - Review your own code first
   - Check for commented-out code
   - Verify no debugging statements
   - Ensure proper error handling

3. **Update documentation:**
   - Update README if needed
   - Add/update relevant docs
   - Update CHANGELOG.md

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Related Issues
Closes #issue_number

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] E2E tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added and passing
- [ ] No breaking changes (or documented)

## Screenshots (if applicable)
Add screenshots for UI changes
```

### Review Process

1. **Automated checks must pass:**
   - CI/CD pipeline
   - Tests
   - Linting
   - Security scanning

2. **Code review required:**
   - At least 1 approval for small changes
   - At least 2 approvals for significant changes
   - Architecture review for major changes

3. **Merge requirements:**
   - All conversations resolved
   - Squash and merge for feature branches
   - Rebase for hotfixes

## Security

### Reporting Vulnerabilities

**DO NOT** create public issues for security vulnerabilities.

Instead:
1. Email security@lis-modern.com
2. Include detailed description
3. Provide steps to reproduce
4. Allow time for fix before disclosure

### Security Best Practices

- Never commit secrets or credentials
- Use environment variables for configuration
- Validate and sanitize all inputs
- Use parameterized queries
- Implement proper authentication and authorization
- Keep dependencies updated
- Run security scanners regularly

## Questions?

- **Technical questions**: Open a GitHub Discussion
- **Bug reports**: Open a GitHub Issue
- **Feature requests**: Open a GitHub Issue with `enhancement` label
- **General inquiries**: contact@lis-modern.com

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project website (if applicable)

Thank you for contributing to LIS Modern! 🎉
