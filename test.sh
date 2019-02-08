#!/bin/bash
#      ^^^^ - NOT /bin/sh, as this code uses arrays
#
# Run this script
# bash testing.sh <IP_ADDRESS> <NUMBER_OF_NODES>

PIDS=()

# define cleanup function
cleanup() {
  for pid in "${PIDS[@]}"; do
    kill -0 "$pid" && kill "$pid" # kill process only if it's still running
  done
}

# and set that function to run before we exit, or specifically when we get a SIGTERM
trap cleanup EXIT SIGTERM

PROJECT_BUILD="/target/debug/hll-rust"

TEST_IP=$1
NUMBER_OF_NODES=$2

cargo build

.${PROJECT_BUILD} ${TEST_IP} 10001 > /dev/null 2>&1 & PIDS+=( "$!" )
for i in `seq 2 ${NUMBER_OF_NODES}`
do
    sleep .5
    if (($i == $NUMBER_OF_NODES))
    then
        .${PROJECT_BUILD} ${TEST_IP} $((10000+$i)) ${TEST_IP}:10001 & PIDS+=( "$!" )
    else
        .${PROJECT_BUILD} ${TEST_IP} $((10000+$i)) ${TEST_IP}:10001 > /dev/null 2>&1 & PIDS+=( "$!" )
    fi
done

wait