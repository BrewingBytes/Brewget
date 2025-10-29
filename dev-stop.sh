#!/bin/bash
# Stop all development services

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🛑 Stopping BrewGet Development Services${NC}"

# Function to safely kill a process
safe_kill() {
    local pid_file=$1
    local service_name=$2
    
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        # Validate PID is a number
        if [[ "$pid" =~ ^[0-9]+$ ]]; then
            # Check if process exists before killing
            if ps -p "$pid" > /dev/null 2>&1; then
                echo -e "Stopping $service_name..."
                kill "$pid" 2>/dev/null || true
            fi
        fi
        rm -f "$pid_file"
    fi
}

# Stop cargo-watch processes
safe_kill .dev-logs/email.pid "Email Service"
safe_kill .dev-logs/auth.pid "Auth Service"
safe_kill .dev-logs/settings.pid "Settings Service"
safe_kill .dev-logs/frontend.pid "Frontend"

# Also kill any remaining cargo-watch processes
pkill -f "cargo watch" || true

# Also kill any remaining npm/vite processes
pkill -f "vite" || true

echo -e "${GREEN}✅ All services stopped${NC}"
