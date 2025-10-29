#!/bin/bash
# Stop all development services

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🛑 Stopping BrewGet Development Services${NC}"

# Stop cargo-watch processes
if [ -f .dev-logs/email.pid ]; then
    echo -e "Stopping Email Service..."
    kill $(cat .dev-logs/email.pid) 2>/dev/null || true
    rm -f .dev-logs/email.pid
fi

if [ -f .dev-logs/auth.pid ]; then
    echo -e "Stopping Auth Service..."
    kill $(cat .dev-logs/auth.pid) 2>/dev/null || true
    rm -f .dev-logs/auth.pid
fi

if [ -f .dev-logs/settings.pid ]; then
    echo -e "Stopping Settings Service..."
    kill $(cat .dev-logs/settings.pid) 2>/dev/null || true
    rm -f .dev-logs/settings.pid
fi

if [ -f .dev-logs/frontend.pid ]; then
    echo -e "Stopping Frontend..."
    kill $(cat .dev-logs/frontend.pid) 2>/dev/null || true
    rm -f .dev-logs/frontend.pid
fi

# Also kill any remaining cargo-watch processes
pkill -f "cargo watch" || true

# Also kill any remaining npm/vite processes
pkill -f "vite" || true

echo -e "${GREEN}✅ All services stopped${NC}"
