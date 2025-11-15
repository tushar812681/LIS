# =============================================================================
# LIS Modern Backend - Makefile
# =============================================================================
# Common development and deployment tasks
# =============================================================================

.PHONY: help
.DEFAULT_GOAL := help

# =============================================================================
# Configuration
# =============================================================================
RUST_VERSION := 1.75
SERVICES := patient-service sample-service order-service result-service user-service \
            organization-service equipment-service qc-service billing-service \
            report-service inventory-service notification-service analytics-service \
            compliance-service

DOCKER_REGISTRY := ghcr.io/your-org
IMAGE_TAG := latest

# =============================================================================
# Help
# =============================================================================
help: ## Show this help message
	@echo '════════════════════════════════════════════════════════════'
	@echo 'LIS Modern Backend - Available Commands'
	@echo '════════════════════════════════════════════════════════════'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo '════════════════════════════════════════════════════════════'

# =============================================================================
# Development
# =============================================================================
install: ## Install Rust and dependencies
	@echo "Installing Rust toolchain..."
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	@echo "Installing additional components..."
	rustup component add rustfmt clippy
	@echo "Installing cargo tools..."
	cargo install cargo-watch cargo-edit cargo-audit sqlx-cli

setup: install ## Setup development environment
	@echo "Creating .env file..."
	cp .env.example .env
	@echo "Starting infrastructure services..."
	docker-compose up -d postgres redis
	@echo "Waiting for services to be ready..."
	sleep 5
	@echo "Running database migrations..."
	$(MAKE) migrate
	@echo "✅ Setup complete!"

deps: ## Install dependencies
	cargo fetch

build: ## Build all services
	@echo "Building workspace..."
	cargo build --workspace

build-release: ## Build all services in release mode
	@echo "Building workspace in release mode..."
	cargo build --workspace --release

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

# =============================================================================
# Testing
# =============================================================================
test: ## Run all tests
	@echo "Running tests..."
	cargo test --workspace --all-features

test-unit: ## Run unit tests only
	cargo test --workspace --lib

test-integration: ## Run integration tests
	cargo test --workspace --test '*'

test-coverage: ## Generate test coverage report
	cargo tarpaulin --workspace --all-features --timeout 300 --out Html --engine llvm
	@echo "Coverage report generated: target/tarpaulin/index.html"

bench: ## Run benchmarks
	cargo bench --workspace

# =============================================================================
# Code Quality
# =============================================================================
fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

lint: ## Run clippy linter
	cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-fix: ## Auto-fix linting issues
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty

audit: ## Run security audit
	cargo audit

check: fmt-check lint test ## Run all checks (format, lint, test)

fix: fmt lint-fix ## Auto-fix formatting and linting issues

# =============================================================================
# Database
# =============================================================================
db-up: ## Start database services
	docker-compose up -d postgres redis mongodb kafka

db-down: ## Stop database services
	docker-compose down

db-reset: ## Reset databases (WARNING: destroys all data)
	docker-compose down -v
	docker-compose up -d postgres redis mongodb kafka
	sleep 5
	$(MAKE) migrate

migrate: ## Run database migrations
	@echo "Running migrations for all services..."
	@for service in $(SERVICES); do \
		if [ -d "services/$$service/migrations" ]; then \
			echo "Migrating $$service..."; \
			cd services/$$service && sqlx migrate run || true; \
			cd ../..; \
		fi \
	done

migrate-revert: ## Revert last migration
	@echo "Reverting last migration..."
	sqlx migrate revert

# =============================================================================
# Running Services
# =============================================================================
run-patient: ## Run patient service
	cargo run --package patient-service

run-sample: ## Run sample service
	cargo run --package sample-service

run-all: ## Run all services (requires tmux)
	./start_all_services.sh

watch: ## Run service with auto-reload (specify SERVICE=name)
	@if [ -z "$(SERVICE)" ]; then \
		echo "Usage: make watch SERVICE=patient-service"; \
		exit 1; \
	fi
	cargo watch -x "run --package $(SERVICE)"

# =============================================================================
# Docker
# =============================================================================
docker-build: ## Build Docker images for all services
	@for service in $(SERVICES); do \
		echo "Building $$service..."; \
		docker build \
			--build-arg SERVICE_NAME=$$service \
			--build-arg VERSION=$(IMAGE_TAG) \
			-f Dockerfile.production \
			-t $(DOCKER_REGISTRY)/$$service:$(IMAGE_TAG) \
			.; \
	done

docker-push: ## Push Docker images to registry
	@for service in $(SERVICES); do \
		echo "Pushing $$service..."; \
		docker push $(DOCKER_REGISTRY)/$$service:$(IMAGE_TAG); \
	done

docker-run: ## Run services with docker-compose
	docker-compose up -d

docker-stop: ## Stop docker-compose services
	docker-compose down

docker-logs: ## Show docker-compose logs
	docker-compose logs -f

# =============================================================================
# Kubernetes
# =============================================================================
k8s-deploy-dev: ## Deploy to development Kubernetes
	kubectl apply -k ../infrastructure/kubernetes/overlays/dev

k8s-deploy-prod: ## Deploy to production Kubernetes
	kubectl apply -k ../infrastructure/kubernetes/overlays/prod

k8s-status: ## Check Kubernetes deployment status
	kubectl get pods -n lis-modern
	kubectl get services -n lis-modern

k8s-logs: ## Show Kubernetes logs (specify SERVICE=name)
	@if [ -z "$(SERVICE)" ]; then \
		echo "Usage: make k8s-logs SERVICE=patient-service"; \
		exit 1; \
	fi
	kubectl logs -f -n lis-modern deployment/$(SERVICE)

k8s-shell: ## Open shell in Kubernetes pod (specify SERVICE=name)
	@if [ -z "$(SERVICE)" ]; then \
		echo "Usage: make k8s-shell SERVICE=patient-service"; \
		exit 1; \
	fi
	kubectl exec -it -n lis-modern deployment/$(SERVICE) -- /bin/sh

# =============================================================================
# Documentation
# =============================================================================
docs: ## Generate and open documentation
	cargo doc --workspace --no-deps --open

docs-build: ## Build documentation
	cargo doc --workspace --no-deps

# =============================================================================
# Performance
# =============================================================================
perf-test: ## Run performance tests
	@echo "Running performance tests..."
	node performance-test.js

profile: ## Profile application (requires cargo-flamegraph)
	cargo flamegraph --package patient-service

# =============================================================================
# Utilities
# =============================================================================
health: ## Check health of all running services
	./test_all_health.sh

version: ## Show version information
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Project version: $$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"

update: ## Update dependencies
	cargo update

outdated: ## Check for outdated dependencies
	cargo outdated

tree: ## Show dependency tree
	cargo tree --workspace

size: ## Show binary sizes
	@echo "Binary sizes (release mode):"
	@du -h target/release/* 2>/dev/null | grep -v '\.d$$' || echo "No release binaries found. Run 'make build-release' first."

# =============================================================================
# CI/CD
# =============================================================================
ci-local: ## Run CI checks locally
	@echo "Running CI checks locally..."
	$(MAKE) fmt-check
	$(MAKE) lint
	$(MAKE) test
	$(MAKE) audit
	@echo "✅ All CI checks passed!"

pre-commit: ## Run pre-commit checks
	$(MAKE) fmt
	$(MAKE) lint-fix
	$(MAKE) test-unit

# =============================================================================
# Production
# =============================================================================
prod-build: ## Build for production
	cargo build --workspace --release
	@echo "Stripping binaries..."
	@for service in $(SERVICES); do \
		if [ -f "target/release/$$service" ]; then \
			strip target/release/$$service; \
		fi \
	done
	@echo "✅ Production build complete!"

prod-check: ## Run production readiness checks
	@echo "Checking production readiness..."
	@echo "1. Running security audit..."
	cargo audit
	@echo "2. Checking for TODO/FIXME..."
	@! grep -r "TODO\|FIXME" services/ libs/ || echo "Warning: Found TODO/FIXME in code"
	@echo "3. Checking for unwrap() calls..."
	@! grep -r "\.unwrap()" services/ libs/ || echo "Warning: Found unwrap() calls"
	@echo "4. Checking secrets..."
	@! git secrets --scan || echo "Warning: Potential secrets found"
	@echo "✅ Production checks complete!"

# =============================================================================
# Cleanup
# =============================================================================
clean-all: clean ## Clean everything including Docker volumes
	docker-compose down -v
	rm -rf target/ .env

# =============================================================================
# Quick Commands
# =============================================================================
dev: setup run-all ## Setup and run all services for development

quick-test: ## Quick test (unit tests only, no coverage)
	cargo test --workspace --lib --quiet

# =============================================================================
# END
# =============================================================================
