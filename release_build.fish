#!/usr/bin/env fish

RUSTFLAGS="-C target-cpu=native -C llvm-args=-fp-contract=fast" cargo build --release