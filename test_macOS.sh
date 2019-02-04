#!/bin/sh
PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP="145.97.252.154"

cargo build
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

nohup   .$PROJECT_BUILD -i $TEST_IP -p 10000                  > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10010 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10020 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10030 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10040 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10050 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10060 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
        .$PROJECT_BUILD -i $TEST_IP -p 10070 -j $TEST_IP:10000 && fg