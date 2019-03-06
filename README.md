# hll-rust

High level languages: Rust - Group project (2018/2019): Chord is a protocol and algorithm for a peer-to-peer distributed hash table.

##### CircleCI

[![CircleCI](https://circleci.com/gh/andreasellw/hll-rust.svg?style=shield&circle-token=d3cb985f6b03b2e2a6ef47851da0e5e29fbbf296)](https://circleci.com/gh/andreasellw/hll-rust)

## Table of Contents

- [hll-rust](#hll-rust)
        - [CircleCI](#circleci)
  - [Table of Contents](#table-of-contents)
  - [Demonstration](#demonstration)
  - [Installation](#installation)
    - [Requirements](#requirements)
  - [Usage](#usage)
    - [Run a single node](#run-a-single-node)
    - [Open menu dialog](#open-menu-dialog)
    - [Spawn multiple nodes at once](#spawn-multiple-nodes-at-once)
      - [Important notes](#important-notes)
  - [Crates](#crates)
  - [Chord](#chord)
    - [References](#references)
  - [Contributors (Group E)](#contributors-group-e)
    - [Individual contributions](#individual-contributions)

## Demonstration

In the following demo first `./target/debug/hll-rust 10.0.1.2 11111` was executed.
Soon afterwards `bash test.sh 10.0.1.2 10000 10 10.0.1.2:11111` was executed in another terminal.
Take a look the terminal of the `11111` node.

[![demo](https://asciinema.org/a/226513.svg)](https://asciinema.org/a/226513?autoplay=1)

## Installation

Run in the project folder `cargo build`.

### Requirements

```bash
$ rustc --version
rustc 1.32.0 (9fda7c223 2019-01-16)
$ rustup --version
rustup 1.16.0 (beab5ac2b 2018-12-06)
$ cargo --version
cargo 1.32.0 (8610973aa 2019-01-02)
```

## Usage

**Important Disclaimer**: 
When we speak of **LocalIp4Addr** in the following parts of the readme we mean the IPV4 Address you have in your local network (e.g. eduroam) **not** localhost, this can be found out by calling *ifconfig* in a terminal

### Run a single node

To print our CLI help run  which prints

```bash
$ cargo run -- -h

hll_rust_chord 1.0
Andreas Ellwanger, Timo Erdelt and Andreas Griesbeck
High level languages: Rust - Group project (2018/2019)

USAGE:
    hll-rust <IP4ADDR> <PORT> [IP4ADDR:PORT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <IP4ADDR>         Sets the ip address to use (e.g. 127.0.0.1)
    <PORT>            Sets the port to use
    <IP4ADDR:PORT>    Sets the node (entry point to an existing chord ring) to join
```

To spawn a single node which creates a new chord ring run

```bash
cargo run -- <LocalIp4Addr> <LocalPort>
```

To spawn a single node and join an existing chord ring run

```bash
cargo run -- <LocalIp4Addr> <LocalPort> <OtherIp4Addr:OtherPort>
```

### Open menu dialog

To open the menu while running a node type `m` and press `ENTER` which opens the following menu

```text
Hello there! What do you want to do?

1 - Store a key/value pair in the Chord network
2 - Find the value for a given key in the Chord network
3 - Delete a key/value pair from the Chord network

4 - Kill a Chord network peer

5 - Cancel interaction
6 - Terminate Node

Choose 1, 2, 3, 4, 5 or 6 and press Enter!
```

- To store a key and value within the chord ring press `1+ENTER` and follow the instructions.
- To find a value within the chord ring press `2+ENTER` and follow the instructions.
- To delete a key and value within the chord ring press `3+ENTER` and follow the instructions.
- To kill another chord ring peer press `4+ENTER` and follow the instructions.
- To close the menu press `5+ENTER`.
- To terminate the node press `6+ENTER`.

### Spawn multiple nodes at once

In order to spawn a new chord ring with a given number of nodes on a system we created a bash script which can be used as follows:

```bash
bash test.sh <LocalIp4Addr> <PortOfFirstNode> <NumberOfNodes>
```

In order to create a number of nodes and join them on an existing chord ring the same script can be used by applying an additional command line argument:

```bash
bash test.sh <LocalIp4Addr> <PortOfFirstNode> <NumberOfNodes> <IpOfJoinNode:Port>
```

#### Important notes

- The script creates the nodes with ports starting at `<PortOfFirstNode>` and ending at `<PortOfFirstNode+NumberOfNodes-1>`
- Unfortunately the menu for interacting with the chord ring (e.g story, querying, deleting from DHT) does not work with the node being rendered after the script has completed. In order to get the menu a new node has to be spawned in another terminal with one of the IP addresses of the just spawned ring as the join IP.
- We have not tested the script big number of nodes, we usually ran it with 10 nodes which did produce no problems, but it should theoretically also work for a bigger amounts, but we sometimes ran into problems running more nodes on a single machine

## Crates

```text
chrono, clap, colored, futures, get_if_addrs, log, log4rs, num,
num-bigint, prettytable-rs, rust-crypto, serde, serde_derive,
serde_json, signal-hook, tokio
```

For more details take a look at the [Cargo.toml](Cargo.toml).

## Chord

Take a look at our [chord description](CHORD.md) or our [references](#references).

### References

- [https://github.com/sit/dht/wiki](https://github.com/sit/dht/wiki)
- [https://sarwiki.informatik.hu-berlin.de/Chord](https://sarwiki.informatik.hu-berlin.de/Chord)
- [https://dl.acm.org/citation.cfm?doid=964723.383071](https://dl.acm.org/citation.cfm?doid=964723.383071)
- [https://en.wikipedia.org/wiki/Chord_(peer-to-peer)](https://en.wikipedia.org/wiki/Chord_(peer-to-peer))
- [http://nms.csail.mit.edu/papers/chord.pdf](http://nms.csail.mit.edu/papers/chord.pdf)
- [https://pdos.csail.mit.edu/papers/ton:chord/paper-ton.pdf](https://pdos.csail.mit.edu/papers/ton:chord/paper-ton.pdf)

## Contributors (Group E)

- Andreas Ellwanger
- Timo Erdelt
- Andreas Griesbeck

### Individual contributions

Due too the small group size of 3 it is impossible for us to properly distinguish what of our project has been done by whom. We all worked on all parts of our application, especially since we mostly did “pair-programming” (with two or often all three of us working together). So all of us were equally involved in all parts of our application, namely networking (TCP sockets, Tokio), threading, algorithm & communication protocol, user interaction through the command line interface & bash scripts.
We would be happy to answer questions about our development process, aswell as our individual/colletive contributions at the examination.
