# Conway's Steinway Logging Module

This module provides a flexible, multi-destination logging system for the Conway's Steinway application. It's built on top of the Rust `log` and `log4rs` crates and supports a wide range of logging destinations.

## Features

- Multiple logging destinations in a single configuration
- Configurable log levels for each destination
- File rotation with size and count limits
- JSON-formatted logging
- Network logging to various endpoints
- Database logging
- Message queue logging

## Supported Logging Destinations

### Basic Destinations (Always Available)

- **Console**: Log to the console (stdout)
- **File**: Log to a file with optional rotation
- **Json**: Log to a file in JSON format with optional rotation

### Network Destinations (Feature-Gated)

- **HTTP** (`http` feature): Send logs to any HTTP/HTTPS endpoint
- **Syslog** (`syslog` feature): Log to local or remote syslog servers
- **Socket** (`socket` feature): Send logs to a TCP or UDP socket
- **Fluentd** (`fluentd` feature): Send logs to Fluentd data collector
- **GELF** (`gelf` feature): Send logs in Graylog Extended Log Format

### Database Destinations (Feature-Gated)

- **MongoDB** (`mongodb` feature): Log to a MongoDB database
- **PostgreSQL** (`postgres` feature): Log to a PostgreSQL database

### Message Queue Destinations (Feature-Gated)

- **Kafka** (`kafka` feature): Send logs to Apache Kafka
- **RabbitMQ** (`rabbitmq` feature): Send logs to RabbitMQ
- **Redis** (`redis` feature): Send logs to Redis (list, pub/sub, or channel)

## Configuration

Logging destinations are configured through the application's configuration system. Here's an example configuration with multiple destinations:

```yaml
log_destinations:
  - name: "console"
    destination_type: "Console"
    level: "info"
    pattern: "[{h({l})}] {m}{n}"
  
  - name: "file"
    destination_type: "File"
    level: "debug"
    file_path: "logs/backend/app.log"
    rotation:
      enabled: true
      size_limit: 10485760  # 10 MB
      file_count: 5
      
  - name: "json"
    destination_type: "Json"
    level: "info"
    file_path: "logs/backend/app.json"
    
  - name: "http"
    destination_type: "Http"
    level: "warn"
    http:
      url: "https://logging.example.com/api/logs"
      method: "POST"
      content_type: "application/json"
      headers:
        X-API-Key: "your-api-key"
      
  - name: "syslog"
    destination_type: "Syslog"
    level: "info"
    syslog:
      hostname: "syslog.example.com"
      port: 514
      protocol: "udp"
      facility: "local0"
      app_name: "conways-steinway"
```

## Enabling Features

To use network, database, or message queue appenders, you need to enable the corresponding features in your Cargo.toml:

```toml
[dependencies]
logging = { version = "0.1.0", features = ["http", "syslog"] }
```

You can also use the convenience feature groups:

```toml
[dependencies]
logging = { version = "0.1.0", features = ["all-network"] }
```

Available feature groups:
- `all-network`: Enables all network appenders
- `all-database`: Enables all database appenders
- `all-messageq`: Enables all message queue appenders
- `all`: Enables all appenders

## Logging Patterns

All destinations that support text formatting (Console, File, Syslog, etc.) can use a custom pattern string. Here are some examples:

- `[{l}] {m}{n}` - Simple format with level and message
- `[{d(%Y-%m-%d %H:%M:%S)} {l}] {t} - {m}{n}` - Full format with date, level, thread, and message
- `{m}{n}` - Message only

See the [log4rs documentation](https://docs.rs/log4rs/) for more pattern options.