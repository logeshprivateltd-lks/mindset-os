#!/bin/sh
# Wrapper delegating to canonical build-iso.sh entrypoint
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$SCRIPT_DIR/build-iso.sh" "$@"
