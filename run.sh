#!/bin/bash

# kill lingering processes
pkill lighthouse_rss_bench

cargo build --release


for CASE in "map_pkidx" "rpds_map_pkidx"; do
  for N in 100000 1000000 10000000; do
    > /tmp/lighthouse_rss_bench

    # Start in the background
    N=$N CASE=$CASE ./target/release/lighthouse_rss_bench > /tmp/lighthouse_rss_bench &
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
    rss_value=$(ps x -o rss,comm | grep "target/release/lighthouse_rss_bench" | awk '{print $1}')

    # Check if rss_value is not empty
    if [ -z "$rss_value" ]; then
      echo "Process not found."
      exit 1
    fi

    result=$(echo "scale=4; (1000 * $rss_value) / $N" | bc)
    echo "CASE=$CASE N=$N rss=$rss_value bytes/N=$result"

    # Kill and wait to terminate
    kill $PID > /dev/null 2>&1
    wait $PID > /dev/null 2>&1
  done
done

