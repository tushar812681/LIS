#!/bin/bash

# LIS Modern Backend - Start All Services
# This script starts all 14 microservices in the background

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        LIS Modern Backend - Starting All Services           ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"

# Create logs directory
mkdir -p "$LOG_DIR"

# Service configuration (name:port)
SERVICES=(
    "patient-service:8081"
    "sample-service:8082"
    "order-service:8083"
    "result-service:8084"
    "user-service:8085"
    "organization-service:8086"
    "equipment-service:8087"
    "qc-service:8088"
    "billing-service:8089"
    "report-service:8090"
    "inventory-service:8091"
    "notification-service:8092"
    "analytics-service:8093"
    "compliance-service:8094"
)

# Function to start a service
start_service() {
    local service_name=$1
    local port=$2
    local service_dir="$SCRIPT_DIR/services/$service_name"
    local log_file="$LOG_DIR/${service_name}.log"
    local pid_file="$LOG_DIR/${service_name}.pid"

    echo -e "${YELLOW}Starting $service_name on port $port...${NC}"

    if [ ! -d "$service_dir" ]; then
        echo -e "  ${RED}✗ Service directory not found: $service_dir${NC}"
        return 1
    fi

    # Start the service in background
    cd "$service_dir"
    nohup cargo run --release > "$log_file" 2>&1 &
    local pid=$!
    echo $pid > "$pid_file"

    echo -e "  ${GREEN}✓ Started with PID $pid${NC}"
    echo -e "  ${BLUE}  Log: $log_file${NC}"

    cd "$SCRIPT_DIR"
}

# Start all services
echo -e "${BLUE}Starting all 14 services...${NC}"
echo ""

for service_config in "${SERVICES[@]}"; do
    IFS=':' read -r service_name port <<< "$service_config"
    start_service "$service_name" "$port"
    sleep 2  # Small delay between service starts
done

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              All Services Started Successfully!              ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Service Logs Location: $LOG_DIR${NC}"
echo -e "${BLUE}PID Files Location: $LOG_DIR/*.pid${NC}"
echo ""
echo -e "${YELLOW}Wait 10-15 seconds for all services to initialize...${NC}"
echo ""
echo -e "${BLUE}To check service status:${NC}"
echo "  tail -f $LOG_DIR/<service-name>.log"
echo ""
echo -e "${BLUE}To stop all services:${NC}"
echo "  ./stop_all_services.sh"
echo ""
