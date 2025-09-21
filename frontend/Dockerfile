# Build stage
FROM oven/bun:1 AS builder

WORKDIR /app

# Copy package files
COPY package.json bun.lockb ./

# Install dependencies
RUN bun install

# Copy source code
COPY . .

# Set build environment
ENV NODE_ENV=production

# Build the application
RUN bun run build-only

# Production stage - using lightweight static server
FROM busybox:1.36

# Install basic HTTP server
RUN adduser -D static
USER static
WORKDIR /home/static

# Copy built assets from builder
COPY --from=builder /app/dist .

# Expose port
EXPOSE 80

# Start simple HTTP server
CMD ["busybox", "httpd", "-f", "-v", "-p", "80", "-h", "/home/static"]

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
