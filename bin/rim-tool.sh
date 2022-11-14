#!/bin/bash

cargo run --bin rim-tool -- -i ./samples/chessboard_f32_8x8.bin -d 8x8 -b 32 -p angles.csv -o sinogram.bin
