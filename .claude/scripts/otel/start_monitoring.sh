#!/bin/bash

# Start the OpenTelemetry collector in the background
echo "Starting OpenTelemetry collector..."
cd ~/.claude/scripts/otel
./otelcol --config=config.yaml &
OTEL_PID=$!

# Start the monitoring stack
echo "Starting Prometheus and Grafana..."
docker-compose up -d

echo ""
echo "=========================================================="
echo "Monitoring setup is complete!"
echo "OpenTelemetry Collector is running with PID: $OTEL_PID"
echo ""
echo "Prometheus is available at: http://localhost:19091"
echo "Grafana is available at: http://localhost:13000"
echo "  Username: admin"
echo "  Password: admin"
echo ""
echo "To use Claude Code with OpenTelemetry, run:"
echo "source ~/.claude/scripts/otel/setup_claude_otel.sh"
echo "=========================================================="
