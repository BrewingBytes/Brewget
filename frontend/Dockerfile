# Build stage
FROM oven/bun:1 AS builder

WORKDIR /app

# Copy package files
COPY frontend/package.json frontend/bun.lockb ./

# Install dependencies
RUN bun install

# Copy source code
COPY frontend .

# Copy changelogs
COPY changelogs ../changelogs

# Set build environment
ENV NODE_ENV=production

# Build the application
RUN bun run build-only

# Production stage - using nginx for SPA routing
FROM nginx:alpine

# Copy built assets from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration with SPA routing support
COPY frontend/nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
