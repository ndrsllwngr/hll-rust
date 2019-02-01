#!/bin/sh
# Test file
cargo build
PROJECT_BUILD="/target/debug/hll-rust"

# trap ctrl-c and call ctrl_c()
#trap ctrl_c SIGINT SIGTERM EXIT
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

nohup .$PROJECT_BUILD -i 10.0.1.2 -p 22222 > /dev/null 2>&1 &
.$PROJECT_BUILD -i 10.0.1.2 -p 33333 -j 10.0.1.2:22222 && fg

