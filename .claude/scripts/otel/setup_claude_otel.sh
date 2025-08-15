#!/bin/bash

# Enable telemetry for Claude Code
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# Configure OTLP exporters
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp

# Set OTLP endpoint to local collector
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14317

# Optional: Set additional configuration
export OTEL_SERVICE_NAME=claude-code
export OTEL_RESOURCE_ATTRIBUTES=service.name=claude-code,deployment.environment=local

echo "OpenTelemetry environment variables set for Claude Code"
echo "To use these settings in your current shell, run:"
echo "source ~/.claude/scripts/otel/setup_claude_otel.sh"
