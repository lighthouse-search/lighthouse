#!/usr/bin/env bash
# Starts the API server and the frontend together for local development, and
# tears both down on Ctrl-C. Invoked from the repo root by `make dev`.
set -uo pipefail

API_PORT="${API_PORT:-4459}"
API_ENDPOINT="${API_ENDPOINT:-http://localhost:${API_PORT}}"
FRONTEND_DIR="${FRONTEND_DIR:-frontend/lighthouse}"

pids=()
cleanup() {
    echo ""
    echo "Stopping dev environment..."
    for pid in "${pids[@]}"; do
        kill "$pid" 2>/dev/null || true
    done
}
trap cleanup EXIT INT TERM

echo "Starting API server on port ${API_PORT}..."
( cd server && lighthouse_port="${API_PORT}" cargo run ) &
pids+=("$!")

echo "Starting frontend (API endpoint: ${API_ENDPOINT})..."
( cd "${FRONTEND_DIR}" && NEXT_PUBLIC_api_endpoint="${API_ENDPOINT}" yarn dev ) &
pids+=("$!")

# Block until a process exits; the trap then tears the rest down.
wait
