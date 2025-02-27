#!/bin/sh
set -e

# Function to log messages
log() {
  echo "[$(date -Iseconds)] $1"
}

# Function to cleanup processes
cleanup() {
    log "Cleaning up processes..."
    kill -TERM $NGINX_PID $NODE_PID 2>/dev/null
}

# Setup trap
trap cleanup EXIT

# Start nginx in background
log "Starting nginx..."
nginx -g 'daemon off;' &
NGINX_PID=$!

# Start node server
cd /app/server
log "Starting Node.js server..."
node app.js &
NODE_PID=$!

# Wait for services to be ready
sleep 2

# Check if services are running
if ! kill -0 $NGINX_PID 2>/dev/null; then
  log "Error: nginx failed to start"
  exit 1
fi

if ! kill -0 $NODE_PID 2>/dev/null; then
  log "Error: Node.js server failed to start"
  exit 1
fi

log "All services started successfully"

# Wait for either process to exit
wait -n

# Exit with the same code as the failed process
exit 1 