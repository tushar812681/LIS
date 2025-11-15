#!/bin/bash

# LIS Modern Backend - Quick Start All Services
# Uses cargo run (builds incrementally and starts faster)

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     LIS Modern - Quick Start (14 Services)                  ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

# Source cargo environment
export PATH="$HOME/.cargo/bin:$PATH"

# Services array
SERVICES=(
    "user-service:8085"
    "organization-service:8086"
    "patient-service:8081"
    "sample-service:8082"
    "order-service:8083"
    "result-service:8084"
    "equipment-service:8087"
    "inventory-service:8091"
    "qc-service:8088"
    "billing-service:8089"
    "notification-service:8092"
    "analytics-service:8093"
    "report-service:8090"
    "compliance-service:8094"
)

start_service() {
    local service_name=$1
    local port=$2
    local service_dir="$SCRIPT_DIR/services/$service_name"
    local log_file="$LOG_DIR/${service_name}.log"
    local pid_file="$LOG_DIR/${service_name}.pid"

    echo -e "${YELLOW}[$service_name] Starting on port $port...${NC}"

    cd "$service_dir"
    nohup cargo run --release 2>&1 | while IFS= read -r line; do
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] $line"
    done > "$log_file" &
    local pid=$!
    echo $pid > "$pid_file"

    echo -e "  ${GREEN}✓ Started (PID: $pid)${NC}"
    cd "$SCRIPT_DIR"
}

echo -e "${BLUE}Starting all services (builds incrementally)...${NC}"
echo -e "${YELLOW}This will take 2-5 minutes for first-time builds${NC}"
echo ""

for service_config in "${SERVICES[@]}"; do
    IFS=':' read -r service_name port <<< "$service_config"
    start_service "$service_name" "$port"
    sleep 1
done

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║         All Services Started - Building in Background!      ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}⏳ Services are building and will be ready in 2-5 minutes${NC}"
echo -e "${BLUE}📋 Logs: $LOG_DIR/*.log${NC}"
echo -e "${BLUE}🔍 Monitor: tail -f $LOG_DIR/<service>.log${NC}"
echo -e "${BLUE}🛑 Stop All: ./stop_all_services.sh${NC}"
echo ""
