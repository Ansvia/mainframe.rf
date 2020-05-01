#!/usr/bin/env bash

export RUST_BACKTRACE=1
export RUST_LOG=$name_snake_case$=debug

cargo run --bin $name_snake_case$_server

