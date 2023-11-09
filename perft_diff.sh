#!/bin/bash
# your-perft.sh

# The first argument is the depth, the second argument is the FEN, the third argument is moves (not mendotory)
# https://github.com/agausmann/perftree
./target/release/persa_chess "$1" "$2" "$3"
