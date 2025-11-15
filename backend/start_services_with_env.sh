#!/bin/bash

# LIS Modern Backend - Start All Services with Environment Variables

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     LIS Modern - Starting Services with Configuration       ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

# Source cargo environment
export PATH="$HOME/.cargo/bin:$PATH"

# Common environment variables
export RUST_LOG=info
export RUST_BACKTRACE=1
export JWT_SECRET="your-super-secret-jwt-key-change-in-production-min-32-chars"
export REDIS_HOST=localhost
export REDIS_PORT=6379
export REDIS_URL="redis://localhost:6379"
export KAFKA_BROKERS="localhost:9092"

# Start service with specific configuration
start_service() {
    local service_name=$1
    local port=$2
    local database_name=$3
    local service_dir="$SCRIPT_DIR/services/$service_name"
    local log_file="$LOG_DIR/${service_name}.log"
    local pid_file="$LOG_DIR/${service_name}.pid"

    echo -e "${YELLOW}[$service_name] Starting on port $port...${NC}"

    if [ ! -d "$service_dir" ]; then
        echo -e "  ${RED}✗ Service directory not found${NC}"
        return 1
    fi

    # Set service-specific environment
    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/$database_name"
    export PORT=$port

    cd "$service_dir"
    nohup cargo run --release 2>&1 | while IFS= read -r line; do
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] $line"
    done > "$log_file" &
    local pid=$!
    echo $pid > "$pid_file"

    echo -e "  ${GREEN}✓ Started (PID: $pid, DB: $database_name)${NC}"
    cd "$SCRIPT_DIR"
}

echo -e "${BLUE}Starting all 14 services with proper configuration...${NC}"
echo ""

# Start services with their specific databases
start_service "user-service" 8085 "lis_user"
sleep 2

start_service "organization-service" 8086 "lis_organization"
sleep 2

start_service "patient-service" 8081 "lis_patient"
sleep 2

start_service "sample-service" 8082 "lis_sample"
sleep 2

start_service "order-service" 8083 "lis_order"
sleep 2

start_service "result-service" 8084 "lis_result"
sleep 2

start_service "equipment-service" 8087 "lis_equipment"
sleep 2

start_service "inventory-service" 8091 "lis_inventory"
sleep 2

start_service "qc-service" 8088 "lis_qc"
sleep 2

start_service "billing-service" 8089 "lis_billing"
sleep 2

start_service "notification-service" 8092 "lis_inventory"
sleep 2

start_service "analytics-service" 8093 "lis_inventory"
sleep 2

start_service "report-service" 8090 "lis_report"
sleep 2

start_service "compliance-service" 8094 "lis_inventory"
sleep 2

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║         All Services Started with Configuration!             ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}⏳ Services are starting (already compiled, should be ready in 30-60s)${NC}"
echo -e "${BLUE}📋 Logs: $LOG_DIR/*.log${NC}"
echo -e "${BLUE}🔍 Monitor: tail -f $LOG_DIR/<service>.log${NC}"
echo -e "${BLUE}🛑 Stop All: ./stop_all_services.sh${NC}"
echo ""
