#!/bin/sh

#
# run this as follows:
# sh ./test_macOS.sh IP_ADDRESS NUMBER_OF_NODES
#
PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP=$1
NUMBER_OF_NODES=$2

trap 'quit' 2 15

quit()
{
  echo "Caught SIGINT / SIGTERM ...exiting now."
  exit 1
}


cargo build

nohup   .${PROJECT_BUILD} ${TEST_IP} 10001 > /dev/null 2>&1 &
for i in `seq 2 ${NUMBER_OF_NODES}`
do
    sleep .5
    if (($i == $NUMBER_OF_NODES))
    then
        .${PROJECT_BUILD} ${TEST_IP} $((10000+$i)) ${TEST_IP}:10001 && fg
    else
        nohup   .${PROJECT_BUILD} ${TEST_IP} $((10000+$i)) ${TEST_IP}:10001 > /dev/null 2>&1 &
    fi
done
