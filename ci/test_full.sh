#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

# num-complex should build and test everywhere.
cargo test --verbose

# It should build with minimal features too.
cargo test --no-default-features

# It should build with std (same as default features)
cargo test --no-default-features --features="std"

# It should build with both serialization library
cargo test --verbose --features="serde"
cargo test --verbose --features="rustc-serialize"
cargo test --verbose --no-default-features --features="serde"
# cargo test --verbose --no-default-features --features="rustc-serialize"

# Downgrade serde and build test the 0.7.0 channel as well
cargo update -p serde --precise 0.7.0
cargo test --verbose --no-default-features --features "serde"
cargo test --verbose --no-default-features --features "std serde"
