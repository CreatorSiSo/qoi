#!/bin/fish

# TODO: Make this actually work or port it to rust code
cargo build -r && time target/release/qoi && echo $CMD_DURATION >> bench.log
