#!/bin/bash

# Function to check if a process is running
is_running() {
  pgrep -f "$1" > /dev/null
  return $?
}

# Log directory
LOG_DIR="$HOME/.claude/scripts/otel/logs"
mkdir -p "$LOG_DIR"

# Log file with timestamp
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/otel_startup_$TIMESTAMP.log"

echo "Starting OpenTelemetry integration for Claude Code" > "$LOG_FILE"
echo "Timestamp: $(date)" >> "$LOG_FILE"

# Export OpenTelemetry environment variables
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14317
export OTEL_SERVICE_NAME=claude-code
export OTEL_RESOURCE_ATTRIBUTES=service.name=claude-code,deployment.environment=local

# Write environment variables to the log
echo "Environment variables set:" >> "$LOG_FILE"
env | grep -E 'CLAUDE|OTEL' >> "$LOG_FILE"

# Check if OpenTelemetry collector is already running
if is_running "otelcol"; then
  echo "OpenTelemetry collector is already running" >> "$LOG_FILE"
else
  echo "Starting OpenTelemetry collector..." >> "$LOG_FILE"
  cd "$HOME/.claude/scripts/otel"
  ./otelcol --config=config.yaml >> "$LOG_FILE" 2>&1 &
  OTEL_PID=$!
  echo "OpenTelemetry collector started with PID: $OTEL_PID" >> "$LOG_FILE"
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running. Prometheus and Grafana will not be started." >> "$LOG_FILE"
  exit 0
fi

# Check if Prometheus and Grafana are already running
if docker ps | grep -q "prometheus"; then
  echo "Prometheus is already running" >> "$LOG_FILE"
else
  echo "Starting Prometheus and Grafana..." >> "$LOG_FILE"
  cd "$HOME/.claude/scripts/otel"
  docker-compose up -d >> "$LOG_FILE" 2>&1
  echo "Prometheus and Grafana started" >> "$LOG_FILE"
fi

echo "OpenTelemetry integration startup completed" >> "$LOG_FILE"

# Set environment variables for the current session
# This will be picked up by Claude Code
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14317
