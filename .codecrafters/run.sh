#!/bin/sh
#
# This script is used to run your program on CodeCrafters
#
# This runs after .codecrafters/compile.sh
#
# Learn more: https://codecrafters.io/program-interface

# Set RUST_LOG to info to see logs
export RUST_LOG=info

exec /tmp/codecrafters-build-git-rust/release/codecrafters-git "$@"
