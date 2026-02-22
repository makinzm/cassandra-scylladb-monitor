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
