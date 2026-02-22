# cassandra-scylladb-monitor

[![Prometheus CI](https://github.com/makinzm/cassandra-scylladb-monitor/actions/workflows/prometheus-ci.yml/badge.svg)](https://github.com/makinzm/cassandra-scylladb-monitor/actions/workflows/prometheus-ci.yml)

Monitoring stack for Apache Cassandra and ScyllaDB using Prometheus and Grafana.

## Quick Start

### Base stack (Prometheus + Grafana only)

```bash
docker compose -f docker-compose.base.yml up -d
```

| Service    | URL                      | Credentials  |
|------------|--------------------------|--------------|
| Prometheus | http://localhost:9090    | —            |
| Grafana    | http://localhost:3000    | admin / admin |

To log in to Grafana: open http://localhost:3000, enter **admin** / **admin**, then skip or update the password prompt.

### Stop

```bash
docker compose -f docker-compose.base.yml down
```

## Developer Setup

Install tools and activate git hooks after cloning:

```bash
mise install                  # installs lefthook via .mise.toml
mise exec -- lefthook install # activates pre-commit hooks
```

Requires [mise](https://mise.jdx.dev/getting-started.html).

## Scripts

Scripts live under `scripts/`. Always invoke them with `bash` — do not rely on the executable bit.

| Script | What it does |
|--------|-------------|
| `bash scripts/validate-prometheus.sh` | Starts the base stack, waits for Prometheus to be healthy, asserts both `cassandra` and `scylladb` scrape jobs are present, then tears down. Mirrors the Prometheus CI job. |
