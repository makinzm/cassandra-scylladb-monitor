#!/usr/bin/env bash
# Mirrors the Prometheus CI job so you can verify locally before pushing.
# Usage: bash scripts/validate-prometheus.sh
set -euo pipefail

COMPOSE_CMD="docker compose -f docker-compose.base.yml"

cleanup() {
  echo "Stopping base stack..."
  $COMPOSE_CMD down
}
trap cleanup EXIT

echo "==> Starting base stack (Prometheus + Grafana)..."
$COMPOSE_CMD up -d

echo "==> Waiting for Prometheus to become healthy..."
for i in $(seq 1 12); do
  if curl -sf http://localhost:9090/-/healthy > /dev/null; then
    echo "Prometheus is healthy."
    break
  fi
  echo "Attempt $i/12: not ready yet, waiting 5s..."
  sleep 5
  if [ "$i" -eq 12 ]; then
    echo "ERROR: Prometheus did not become healthy within 60 seconds."
    $COMPOSE_CMD logs prometheus
    exit 1
  fi
done

echo "==> Asserting cassandra and scylladb scrape jobs are present..."
for i in $(seq 1 12); do
  targets=$(curl -sf http://localhost:9090/api/v1/targets)
  if echo "$targets" | python3 -c "
import json, sys
data = json.load(sys.stdin)
jobs = {t['labels']['job'] for t in data['data']['activeTargets']}
print('Active jobs found:', jobs)
missing = {'cassandra', 'scylladb'} - jobs
sys.exit(1 if missing else 0)
"; then
    echo "OK: both cassandra and scylladb jobs are present."
    exit 0
  fi
  echo "Attempt $i/12: jobs not ready yet, waiting 5s..."
  sleep 5
done

echo "ERROR: cassandra and/or scylladb scrape jobs did not appear within 60 seconds."
curl -s http://localhost:9090/api/v1/targets | python3 -m json.tool
exit 1
