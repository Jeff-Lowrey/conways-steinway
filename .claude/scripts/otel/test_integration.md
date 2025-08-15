# Testing Claude Code OpenTelemetry Integration

Follow these steps to test if the OpenTelemetry integration is working properly:

## Step 1: Start the monitoring stack

```bash
# Start the OpenTelemetry collector and monitoring tools
~/.claude/scripts/otel/start_monitoring.sh
```

## Step 2: Configure Claude Code to use OpenTelemetry

```bash
# Set the required environment variables
source ~/.claude/scripts/otel/setup_claude_otel.sh
```

## Step 3: Run some Claude Code commands

```bash
# Run a few Claude Code commands to generate telemetry data
claude-code ls
claude-code --help
# Add more commands as needed
```

## Step 4: Verify the data in monitoring tools

1. Check Prometheus (http://localhost:19091):
   - Go to Status > Targets to verify that the OpenTelemetry collector endpoint is being scraped
   - Go to Graph and search for metrics with the prefix `claudecode_`

2. Check Grafana (http://localhost:13000):
   - Log in with admin/admin
   - Create a new dashboard
   - Add panels for metrics with the prefix `claudecode_`

## Troubleshooting

If no metrics appear:
1. Check that the OpenTelemetry collector is running: `ps aux | grep otelcol`
2. Verify the environment variables are set: `env | grep OTEL`
3. Check the collector logs for any errors: `tail -f ~/.claude/scripts/otel/otelcol.log`
