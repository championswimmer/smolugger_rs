#!/usr/bin/env sh

cargo clean
cargo rustc --bin target -- --emit asm
cp -rf target/debug/deps/target*.s target/debug/target.s
objdump --dwarf=decodedline target/debug/target > target/debug/target.elf
