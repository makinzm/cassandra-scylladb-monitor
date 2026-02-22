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
├── scripts/
│   └── validate-prometheus.sh     # Local smoke test for Prometheus scrape config
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

## Toolchain Management

All project tools are declared in `.mise.toml` and must be invoked via `mise exec --`. This ensures every contributor and CI runner uses the same pinned versions. Docker and kubectl are the only exceptions (assumed pre-installed globally).

```bash
mise install           # install all tools declared in .mise.toml
mise exec -- <cmd>     # run any project tool through the managed environment
```

## Rust Client Commands

```bash
# Run from rust-client/
mise exec -- cargo run    # connects to DB_HOST and executes sample queries
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

## Script Policy

Never use `chmod +x` on scripts. Executable bits are fragile across environments (different OSes, git configs, CI runners). Always invoke scripts explicitly with the interpreter:

```bash
bash scripts/validate-prometheus.sh
```

A lefthook pre-commit hook enforces this: any staged file with mode `100755` will block the commit. After cloning, install tools via mise and activate the hook:

```bash
# Install mise if not already present: https://mise.jdx.dev/getting-started.html
mise install                  # installs lefthook (declared in .mise.toml)
mise exec -- lefthook install # activates the pre-commit hook
```

If a commit is blocked, fix the offending file with:

```bash
git update-index --chmod=-x <file>
```

## Comment Policy

Comment only when the reason behind a decision is non-obvious. Keep comments concise: one or two lines stating the reason and, where applicable, a link to the official documentation or source.

**Good:**
```yaml
# 15s is the Prometheus-recommended default for most workloads.
# https://prometheus.io/docs/prometheus/latest/configuration/configuration/#scrape_config
scrape_interval: 15s
```

**Bad** (restates what the code says, no reference):
```yaml
scrape_interval: 15s  # sets the scrape interval to 15 seconds
```

## Commit Message Policy

Use a single-line conventional commit by default: `<type>: <what and why in one sentence>`.

Add a multi-line body only when the reason is genuinely hard to understand without it (e.g. non-obvious trade-offs, workarounds for upstream bugs).

Types: `feat`, `fix`, `docs`, `chore`, `refactor`.

**Granularity:** Each commit should cover one logical change. Do not bundle tooling setup, new scripts, CI changes, and documentation into a single commit — split them so each commit is independently understandable and revertable.

**Good (single line):**
```
feat: add docker-compose.base.yml as the shared base for all DB stacks
```

**Only if truly needed (multi-line):**
```
feat: add docker-compose.base.yml as the shared base for all DB stacks

<reason that cannot be expressed in one line>
```

## Pull Request Policy

**One PR solves one problem.** Each pull request must correspond to exactly one GitHub issue or one cohesive concern. Do not bundle unrelated changes.

**Grouping guideline:**
- Claude configuration changes (`CLAUDE.md`, `.claude/skills/**`) may be combined in one PR since they form a single concern.
- Project code changes (Docker Compose files, Prometheus config, Grafana dashboards, Rust client) must each be in their own PR tied to a specific issue.

When creating a PR, always reference the issue it closes with `Closes #N` in the commit message and PR body.
