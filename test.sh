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
STARTING_PORT=$2
NUMBER_OF_NODES=$3
JOIN_IP=$4

cargo build

if [[ -z "$JOIN_IP" ]]
    then
        echo "Creating a new chord circle with $NUMBER_OF_NODES nodes..."
        .${PROJECT_BUILD} ${TEST_IP} ${STARTING_PORT} > /dev/null 2>&1 & PIDS+=( "$!" )
        for i in `seq 1 $(($NUMBER_OF_NODES-1))`
        do
            sleep .5
            if (($i == $(($NUMBER_OF_NODES-1))))
            then
                .${PROJECT_BUILD} ${TEST_IP} $(($STARTING_PORT+$i)) ${TEST_IP}:${STARTING_PORT} & PIDS+=( "$!" )
            else
                .${PROJECT_BUILD} ${TEST_IP} $(($STARTING_PORT+$i)) ${TEST_IP}:${STARTING_PORT} > /dev/null 2>&1 & PIDS+=( "$!" )
            fi
        done
    else
        echo "Creating $NUMBER_OF_NODES Nodes and joining them on $JOIN_IP..."
        for i in `seq 0 $(($NUMBER_OF_NODES-1))`
        do
            sleep .5
            if (($i == $(($NUMBER_OF_NODES-1))))
            then
                .${PROJECT_BUILD} ${TEST_IP} $(($STARTING_PORT+$i)) ${JOIN_IP} & PIDS+=( "$!" )
            else
                .${PROJECT_BUILD} ${TEST_IP} $(($STARTING_PORT+$i)) ${JOIN_IP} > /dev/null 2>&1 & PIDS+=( "$!" )
            fi
        done
    fi

wait