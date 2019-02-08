#!/bin/bash
#      ^^^^ - NOT /bin/sh, as this code uses arrays

PIDS=()

# define cleanup function
cleanup() {
  for pid in "${PIDS[@]}"; do
    kill -0 "$pid" && kill "$pid" # kill process only if it's still running
  done
}

# and set that function to run before we exit, or specifically when we get a SIGTERM
trap cleanup EXIT SIGTERM

# f1/p1.py > logs/p1.txt & PIDS+=( "$!" )
# f2/p2.sh > logs/p2.txt & PIDS+=( "$!" )
# nodemon f3/p3.js > logs/p3.txt & PIDS+=( "$!" )

 # sleep until all background processes have exited, or a trap fires


#
# run this as follows:
# sh ./test_macOS.sh IP_ADDRESS NUMBER_OF_NODES
#


PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP=$1
NUMBER_OF_NODES=$2

# trap 'quit' 0 2 15

# quit()
# {
#   echo "Caught SIGINT / SIGTERM ...exiting now."
#   trap - SIGTERM && kill -- -$$
# }


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