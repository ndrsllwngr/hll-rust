#!/bin/sh
# macOS only flush routing table
PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP="10.0.1.2"
INTERFACE="en0"

sudo ifconfig $INTERFACE down
echo "Interface down"
sleep 1
sudo route flush
echo "Route flush"
sleep 1
sudo ifconfig $INTERFACE up
echo "Interface up"
sleep 2

cargo build
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

nohup   .$PROJECT_BUILD -i $TEST_IP -p 8080                   > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 9090 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10010 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10020 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10030 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10040 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10050 -j $TEST_IP:8080 > /dev/null 2>&1 &
sleep .5
        .$PROJECT_BUILD -i $TEST_IP -p 10060 -j $TEST_IP:8080 && fg