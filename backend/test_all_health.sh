#!/bin/bash

echo "=== TESTING ALL SERVICE HEALTH ENDPOINTS ==="
echo ""

PORTS=(8081 8082 8083 8084 8085 8086 8087 8088 8089 8090 8091 8092 8093 8094)
HEALTHY=0
UNHEALTHY=0

for port in "${PORTS[@]}"; do
    result=$(curl -s --max-time 2 http://localhost:$port/health 2>/dev/null)
    if [ -n "$result" ]; then
        service_name=$(echo "$result" | jq -r '.service // "unknown"' 2>/dev/null || echo "running")
        echo "Port $port: ✓ $service_name"
        HEALTHY=$((HEALTHY + 1))
    else
        echo "Port $port: ✗ Not responding"
        UNHEALTHY=$((UNHEALTHY + 1))
    fi
done

echo ""
echo "=== Summary ==="
echo "Healthy services: $HEALTHY/14"
echo "Not responding: $UNHEALTHY/14"
