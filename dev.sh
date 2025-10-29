#!/bin/bash
# Development script to run all services with watch mode

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting BrewGet Development Environment${NC}"

# Check if .env file exists
if [ ! -f .env ]; then
    echo -e "${YELLOW}⚠️  .env file not found. Copying from .env.example...${NC}"
    cp .env.example .env
fi

# Load environment variables
export $(grep -v '^#' .env | xargs)

# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo -e "${YELLOW}📦 Installing cargo-watch...${NC}"
    cargo install cargo-watch
fi

# Create a directory for logs
mkdir -p .dev-logs

echo -e "${GREEN}🔧 Starting microservices with watch mode...${NC}"
echo -e "${YELLOW}Note: Services will automatically restart when code changes are detected${NC}"
echo ""
echo -e "View logs in separate terminals or check .dev-logs/ directory:"
echo -e "  - Auth Service: tail -f .dev-logs/auth-service.log"
echo -e "  - Email Service: tail -f .dev-logs/email-service.log"
echo -e "  - Settings Service: tail -f .dev-logs/settings-service.log"
echo -e "  - Frontend: tail -f .dev-logs/frontend.log"
echo ""

# Start services in background with cargo-watch
cd backend

echo -e "${GREEN}▶️  Starting Email Service (port ${EMAIL_GRPC_PORT:-9001})...${NC}"
(
    cargo watch \
        -w email-service/src \
        -w proto \
        -x "run --package email-service" 
) > ../.dev-logs/email-service.log 2>&1 &
EMAIL_PID=$!

# Wait a bit for email service to start
sleep 3

echo -e "${GREEN}▶️  Starting Auth Service (ports ${AUTH_HTTP_PORT:-8000}, ${AUTH_GRPC_PORT:-9000})...${NC}"
(
    cargo watch \
        -w auth-service/src \
        -w shared-types/src \
        -w proto \
        -x "run --package auth-service"
) > ../.dev-logs/auth-service.log 2>&1 &
AUTH_PID=$!

echo -e "${GREEN}▶️  Starting Settings Service (port ${SETTINGS_HTTP_PORT:-8001})...${NC}"
(
    cargo watch \
        -w settings-service/src \
        -w shared-types/src \
        -w proto \
        -x "run --package settings-service"
) > ../.dev-logs/settings-service.log 2>&1 &
SETTINGS_PID=$!

cd ..

echo -e "${GREEN}▶️  Starting Frontend (port 5173)...${NC}"
(
    cd frontend
    npm run dev
) > .dev-logs/frontend.log 2>&1 &
FRONTEND_PID=$!

# Save PIDs for cleanup
echo "$EMAIL_PID" > .dev-logs/email.pid
echo "$AUTH_PID" > .dev-logs/auth.pid
echo "$SETTINGS_PID" > .dev-logs/settings.pid
echo "$FRONTEND_PID" > .dev-logs/frontend.pid

echo ""
echo -e "${GREEN}✅ All services started!${NC}"
echo ""
echo -e "${GREEN}🌐 Access points:${NC}"
echo -e "  - Frontend: ${GREEN}http://localhost:5173${NC}"
echo -e "  - Auth Service API: ${GREEN}http://localhost:${AUTH_HTTP_PORT:-8000}${NC}"
echo -e "  - Settings Service API: ${GREEN}http://localhost:${SETTINGS_HTTP_PORT:-8001}${NC}"
echo ""
echo -e "${YELLOW}📝 Logs are being written to .dev-logs/${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"

# Trap SIGINT and SIGTERM to cleanup on exit
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Stopping all services...${NC}"
    
    if [ -f .dev-logs/email.pid ]; then
        kill $(cat .dev-logs/email.pid) 2>/dev/null || true
    fi
    if [ -f .dev-logs/auth.pid ]; then
        kill $(cat .dev-logs/auth.pid) 2>/dev/null || true
    fi
    if [ -f .dev-logs/settings.pid ]; then
        kill $(cat .dev-logs/settings.pid) 2>/dev/null || true
    fi
    if [ -f .dev-logs/frontend.pid ]; then
        kill $(cat .dev-logs/frontend.pid) 2>/dev/null || true
    fi
    
    # Clean up PID files
    rm -f .dev-logs/*.pid
    
    echo -e "${GREEN}✅ All services stopped${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Wait for user interrupt
while true; do
    sleep 1
done
