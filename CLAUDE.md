# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A monorepo for monitoring Apache Cassandra and ScyllaDB using Prometheus and Grafana. Cassandra and ScyllaDB are used exclusively — they cannot run simultaneously. A Rust CQL client is included for query execution and validation.

## Repository Structure

```
├── docker-compose.base.yml        # Shared services: Prometheus (9090), Grafana (3000)
├── docker-compose.cassandra.yml   # Cassandra (9042) + JMX Exporter (7070)
├── docker-compose.scylla.yml      # ScyllaDB (9042, metrics on 9180)
├── prometheus/prometheus.yml      # Scrape config (15s interval)
├── grafana/provisioning/
│   ├── datasources/               # Auto-provisioned Prometheus datasource
│   └── dashboards/                # Cassandra and ScyllaDB dashboard JSONs
├── jmx-exporter/cassandra.yml     # JMX export config (Cassandra only)
└── rust-client/
    ├── Cargo.toml
    └── src/main.rs                # CQL client using `scylla` crate
```

## Docker Compose Commands

```bash
# Start with Cassandra
docker compose -f docker-compose.base.yml -f docker-compose.cassandra.yml up -d

# Start with ScyllaDB
docker compose -f docker-compose.base.yml -f docker-compose.scylla.yml up -d

# Stop (use whichever compose combo was used to start)
docker compose -f docker-compose.base.yml -f docker-compose.cassandra.yml down
docker compose -f docker-compose.base.yml -f docker-compose.scylla.yml down
```

## Rust Client Commands

```bash
cd rust-client
cargo build
cargo run                  # runs sample SELECT/INSERT queries
cargo test                 # run all tests
cargo test <test_name>     # run a single test
cargo fmt
cargo clippy
```

The client uses `DB_HOST` env var (default: `localhost:9042`) to set the connection target.

## Key Architecture Decisions

**Metrics ingestion differs by DB:**
- Cassandra: metrics exposed via JMX Exporter sidecar at `:7070/metrics`
- ScyllaDB: native Prometheus endpoint at `:9180/metrics` (no JMX Exporter needed)

**Prometheus** scrapes both jobs at 15s; whichever DB is not running will produce scrape errors (safely ignored).

**Grafana dashboards** are code-managed under `grafana/provisioning/` and auto-loaded on container start. Cassandra and ScyllaDB have separate dashboard JSON files but display the same panel types: p50/p99 latency, RPS (read/write), error rate, and cache hit rate.

**Rust client** uses the `scylla` crate (ScyllaDB driver with Cassandra CQL compatibility) — no instrumentation code, just connectivity and query validation.

## Service URLs

| Service | URL |
|---|---|
| Prometheus | http://localhost:9090 |
| Grafana | http://localhost:3000 (admin/admin) |
| JMX Exporter (Cassandra) | http://localhost:7070/metrics |
| ScyllaDB metrics | http://localhost:9180/metrics |

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `DB_HOST` | `localhost:9042` | DB connection target for Rust client |
| `GF_SECURITY_ADMIN_PASSWORD` | `admin` | Grafana admin password |
