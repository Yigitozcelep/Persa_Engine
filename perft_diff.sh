#!/bin/bash
# your-perft.sh

# The first argument is the depth, the second argument is the FEN.
# We ignore the moves list for simplicity.

# Call your Rust program with the FEN and depth.
# Replace "./target/release/your_perft" with the path to your compiled Rust program.
# https://github.com/agausmann/perftree
./target/release/persa_chess "$1" "$2" "$3"
