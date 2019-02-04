#!/bin/sh

#
# run this as follows:
# sh ./test_macOS.sh IP_ADDRESS NUMBER_OF_NODES
#

PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP=$1
NUMBER_OF_NODES=$2

cargo build
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

nohup   .${PROJECT_BUILD} -i ${TEST_IP} -p 10001 > /dev/null 2>&1 &
for i in `seq 2 ${NUMBER_OF_NODES}`
do
    sleep .5
    if (($i == $NUMBER_OF_NODES))
    then
        .${PROJECT_BUILD} -i ${TEST_IP} -p $((10000+$i)) -j ${TEST_IP}:10001 && fg
    else
        nohup   .${PROJECT_BUILD} -i ${TEST_IP} -p $((10000+$i)) -j ${TEST_IP}:10001 > /dev/null 2>&1 &
    fi
done
