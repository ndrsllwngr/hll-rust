#!/bin/sh
cargo build
PROJECT_BUILD="/target/debug/hll-rust"
TEST_IP="10.0.1.2"
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

nohup   .$PROJECT_BUILD -i $TEST_IP -p 11111                   > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 22222 -j $TEST_IP:11111 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 33333 -j $TEST_IP:11111 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 44444 -j $TEST_IP:11111 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 10000 -j $TEST_IP:11111 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 20000 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
nohup   .$PROJECT_BUILD -i $TEST_IP -p 30000 -j $TEST_IP:10000 > /dev/null 2>&1 &
sleep .5
        .$PROJECT_BUILD -i $TEST_IP -p 10010 -j $TEST_IP:11111 && fg