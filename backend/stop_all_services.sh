#!/bin/bash

# LIS Modern Backend - Stop All Services
# This script stops all running microservices

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        LIS Modern Backend - Stopping All Services           ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"

# Check if logs directory exists
if [ ! -d "$LOG_DIR" ]; then
    echo -e "${RED}No services running (logs directory not found)${NC}"
    exit 0
fi

# Stop services by PID files
STOPPED=0
FAILED=0

for pid_file in "$LOG_DIR"/*.pid; do
    if [ -f "$pid_file" ]; then
        service_name=$(basename "$pid_file" .pid)
        pid=$(cat "$pid_file")

        echo -e "${YELLOW}Stopping $service_name (PID: $pid)...${NC}"

        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null
            sleep 1

            # Force kill if still running
            if kill -0 "$pid" 2>/dev/null; then
                kill -9 "$pid" 2>/dev/null
            fi

            echo -e "  ${GREEN}✓ Stopped${NC}"
            rm -f "$pid_file"
            STOPPED=$((STOPPED + 1))
        else
            echo -e "  ${BLUE}  Already stopped${NC}"
            rm -f "$pid_file"
        fi
    fi
done

# Also kill any cargo run processes for services
echo ""
echo -e "${YELLOW}Cleaning up any remaining service processes...${NC}"
pkill -f "patient-service|user-service|organization-service|sample-service|order-service|result-service|equipment-service|inventory-service|qc-service|billing-service|notification-service|analytics-service|report-service|compliance-service" 2>/dev/null && echo -e "${GREEN}✓ Cleaned up${NC}" || echo -e "${BLUE}  No processes to clean${NC}"

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              All Services Stopped Successfully!              ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ $STOPPED -gt 0 ]; then
    echo -e "${GREEN}Stopped $STOPPED service(s)${NC}"
fi

echo ""
