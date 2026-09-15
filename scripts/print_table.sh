#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/../.env"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Error: .env not found at $ENV_FILE" >&2
  exit 1
fi

set -a
source "$ENV_FILE"
set +a

psql "$DATABASE_URL" -c "SELECT * FROM users;"
psql "$DATABASE_URL" -c "SELECT * FROM chapters;"
psql "$DATABASE_URL" -c "SELECT * FROM novels;"