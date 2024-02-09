#!/bin/bash

cargo clean && cargo rustc -- --crate-type lib --emit=llvm-ir -C opt-level=3