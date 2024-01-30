#!/bin/bash

# Delete existing coverage files
find coverage/ -type f -name '*' -exec rm {} +

# Run tests with coverage
RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="coverage/coverage-%p-%m.profraw" cargo test

# Generate coverage report
grcov . --binary-path ./target/debug/ -s . --ignore-not-existing --ignore "*cargo*" -t lcov > coverage/lcov.info