# Build stage
FROM --platform=$BUILDPLATFORM node:18-alpine AS builder

WORKDIR /app

# Install node-gyp dependencies (needed for some npm packages)
RUN apk add --no-cache python3 make g++

# Copy package files first for better caching
COPY package*.json ./

# Install new dependencies with progress and cache settings
RUN npm install --omit=dev --no-package-lock --progress=false --no-audit \
    && npm cache clean --force

# Copy the rest of the files
COPY . .

# Ensure public directory exists
RUN mkdir -p public

# Build the React app
RUN CI=true npm run build

# Create server dependencies stage
FROM --platform=$BUILDPLATFORM node:18-alpine AS server-deps

WORKDIR /app
COPY server/package*.json ./

# Install server dependencies with production only
RUN npm install --omit=dev --no-package-lock --progress=false --no-audit \
    && npm cache clean --force

# Production stage
FROM nginx:alpine

# Install curl for healthcheck
RUN apk add --no-cache curl

# Install additional dependencies for arm64
RUN case "$(uname -m)" in \
    aarch64) \
      apk add --no-cache nodejs-current ;; \
    *) \
      apk add --no-cache nodejs ;; \
    esac

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy static files
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/build .

# Copy server files and dependencies
COPY server /app/server
COPY --from=server-deps /app/node_modules /app/server/node_modules

# Remove unnecessary files
RUN rm -rf /usr/share/nginx/html/Dockerfile \
    /usr/share/nginx/html/nginx.conf \
    /usr/share/nginx/html/setup.sh \
    /usr/share/nginx/html/README.md \
    /usr/share/nginx/html/k8s

# Copy start script
COPY start.sh /

# Set permissions before switching user
RUN chmod +x /start.sh && \
    chown -R nginx:nginx /start.sh && \
    chown -R nginx:nginx /usr/share/nginx/html && \
    chmod -R 755 /usr/share/nginx/html && \
    chown -R nginx:nginx /var/cache/nginx && \
    chown -R nginx:nginx /var/log/nginx && \
    chown -R nginx:nginx /etc/nginx/conf.d && \
    touch /var/run/nginx.pid && \
    chown -R nginx:nginx /var/run/nginx.pid && \
    chown -R nginx:nginx /app

# Switch to non-root user
USER nginx

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:8080/ || exit 1

CMD ["/start.sh"]
