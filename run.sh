#!/bin/bash
set -e

# kill lingering processes
pkill lighthouse_rss || true

cargo build --release

BEACON_STATE_PATH="/root/holesky_genesis.ssz"

for CASE in "holesky_genesis" "holesky_genesis_build_all_caches" "holesky_genesis_tree_cache" "holesky_genesis_tree_cache_build_all_caches"; do
  for N in 1 2 3; do
    > /tmp/lighthouse_rss_bench

    # Start in the background
    N=$N CASE=$CASE BEACON_STATE_PATH=$BEACON_STATE_PATH ./target/release/lighthouse_rss_bench > /tmp/lighthouse_rss_bench &
    # Capture the last background process's PID
    PID=$!

    # Give the program some time to start and use memory
    while true; do
      if grep -q "allocated" "/tmp/lighthouse_rss_bench"; then
          break
      else
          sleep 1
      fi
    done

    # Note, will error if there are more than one processes running
    # Note: using incomplete name, as ubuntu ps crops the process name
    rss_value=$(ps x -o rss,comm | grep "lighthouse_rss" | awk '{print $1}')

    # Check if rss_value is not empty
    if [ -z "$rss_value" ]; then
      echo "Process not found."
      exit 1
    fi

    result=$(echo "scale=4; (1000 * $rss_value) / $N" | bc)
    echo "CASE=$CASE N=$N rss=$rss_value bytes/N=$result"

    # Kill and wait to terminate
    kill $PID > /dev/null 2>&1 || true
    wait $PID > /dev/null 2>&1 || true
  done
done

