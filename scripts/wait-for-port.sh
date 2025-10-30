#!/bin/bash
# Wait for a TCP port to be available
# Usage: wait-for-port.sh HOST PORT [TIMEOUT]

set -e

HOST=${1:-localhost}
PORT=${2}
TIMEOUT=${3:-30}

if [ -z "$PORT" ]; then
    echo "Usage: $0 HOST PORT [TIMEOUT]"
    exit 1
fi

echo "Waiting for $HOST:$PORT to be available..."

for i in $(seq 1 $TIMEOUT); do
    if timeout 1 bash -c "cat < /dev/null > /dev/tcp/$HOST/$PORT" 2>/dev/null; then
        echo "$HOST:$PORT is available!"
        exit 0
    fi
    sleep 1
done

echo "Timeout waiting for $HOST:$PORT"
exit 1
