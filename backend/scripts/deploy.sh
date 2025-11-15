#!/bin/bash

# LIS Production Deployment Script
# Version: 1.0.0
# Date: 2025-11-15

set -e  # Exit on error

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
COMPOSE_FILE="docker-compose.yml"
ENV_FILE=".env"
BACKUP_DIR="backups"

# Print functions
print_header() {
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo ""
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"

    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed"
        exit 1
    fi
    print_success "Docker is installed ($(docker --version))"

    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed"
        exit 1
    fi
    print_success "Docker Compose is installed ($(docker-compose --version))"

    # Check .env file
    if [ ! -f "$ENV_FILE" ]; then
        print_warning ".env file not found. Creating from template..."
        cp .env.example .env 2>/dev/null || print_error "Template .env.example not found"
        print_info "Please configure .env file before deployment"
        exit 1
    fi
    print_success ".env file exists"

    # Check docker-compose.yml
    if [ ! -f "$COMPOSE_FILE" ]; then
        print_error "docker-compose.yml not found"
        exit 1
    fi
    print_success "docker-compose.yml exists"
}

# Backup databases
backup_databases() {
    print_header "Backing Up Databases"

    mkdir -p "$BACKUP_DIR"
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_FILE="$BACKUP_DIR/lis_backup_$TIMESTAMP.sql"

    print_info "Creating backup: $BACKUP_FILE"

    docker exec lis_postgres pg_dumpall -U postgres > "$BACKUP_FILE" 2>/dev/null || {
        print_warning "Database backup failed (container might not be running)"
        return 0
    }

    print_success "Database backup completed"

    # Keep only last 7 backups
    ls -t "$BACKUP_DIR"/lis_backup_*.sql | tail -n +8 | xargs -r rm
    print_info "Old backups cleaned (keeping last 7)"
}

# Pull latest images
pull_images() {
    print_header "Pulling Latest Images"

    docker-compose pull

    print_success "Images pulled successfully"
}

# Build services
build_services() {
    print_header "Building Services"

    docker-compose build --no-cache

    print_success "Services built successfully"
}

# Start infrastructure
start_infrastructure() {
    print_header "Starting Infrastructure Services"

    docker-compose up -d postgres redis kafka zookeeper minio prometheus grafana jaeger

    print_info "Waiting for infrastructure to be ready..."
    sleep 10

    print_success "Infrastructure services started"
}

# Run migrations
run_migrations() {
    print_header "Running Database Migrations"

    # Wait for PostgreSQL to be ready
    print_info "Waiting for PostgreSQL to be ready..."
    until docker exec lis_postgres pg_isready -U postgres > /dev/null 2>&1; do
        sleep 1
    done
    print_success "PostgreSQL is ready"

    # Migrations are handled by each service on startup
    print_info "Migrations will run automatically on service startup"
}

# Start microservices
start_microservices() {
    print_header "Starting Microservices"

    # Start core services
    print_info "Starting core workflow services..."
    docker-compose up -d patient-service sample-service order-service result-service

    sleep 5

    # Start infrastructure services
    print_info "Starting infrastructure services..."
    docker-compose up -d user-service organization-service

    sleep 5

    # Start operations services
    print_info "Starting operations services..."
    docker-compose up -d equipment-service qc-service billing-service report-service inventory-service

    sleep 5

    # Start support services
    print_info "Starting support services..."
    docker-compose up -d notification-service analytics-service compliance-service

    sleep 5

    # Start new critical services
    print_info "Starting new critical services..."
    docker-compose up -d sync-service file-service integration-service abdm-service

    sleep 5

    # Start API Gateway last
    print_info "Starting API Gateway..."
    docker-compose up -d api-gateway

    print_success "All microservices started"
}

# Health check
health_check() {
    print_header "Running Health Checks"

    # Run test script
    ./scripts/test-all-services.sh || {
        print_error "Health checks failed"
        exit 1
    }

    print_success "All health checks passed"
}

# Show status
show_status() {
    print_header "Deployment Status"

    docker-compose ps

    echo ""
    print_info "Access points:"
    echo "  - API Gateway:      http://localhost:8000"
    echo "  - Grafana:          http://localhost:3001 (admin/admin)"
    echo "  - Prometheus:       http://localhost:9090"
    echo "  - Jaeger:           http://localhost:16686"
    echo "  - MinIO Console:    http://localhost:9001"
}

# Main deployment function
deploy() {
    print_header "LIS Production Deployment"
    echo -e "${YELLOW}Started at: $(date)${NC}"

    check_prerequisites
    backup_databases
    pull_images
    build_services
    start_infrastructure
    run_migrations
    start_microservices
    health_check
    show_status

    print_header "Deployment Complete"
    echo -e "${GREEN}✓ All services deployed successfully!${NC}"
    echo -e "${YELLOW}Completed at: $(date)${NC}"
}

# Parse command line arguments
case "$1" in
    deploy)
        deploy
        ;;
    stop)
        print_header "Stopping All Services"
        docker-compose down
        print_success "All services stopped"
        ;;
    restart)
        print_header "Restarting All Services"
        docker-compose restart
        print_success "All services restarted"
        ;;
    logs)
        docker-compose logs -f "${@:2}"
        ;;
    status)
        docker-compose ps
        ;;
    backup)
        backup_databases
        ;;
    *)
        echo "Usage: $0 {deploy|stop|restart|logs|status|backup}"
        echo ""
        echo "Commands:"
        echo "  deploy   - Deploy all services"
        echo "  stop     - Stop all services"
        echo "  restart  - Restart all services"
        echo "  logs     - Show logs (optional: service name)"
        echo "  status   - Show service status"
        echo "  backup   - Backup databases"
        exit 1
        ;;
esac
