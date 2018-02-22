#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

# num-complex should build and test everywhere.
cargo build --verbose
cargo test --verbose

# It should build with minimal features too.
cargo build --no-default-features
cargo test --no-default-features

# It should build with std
cargo build --no-default-features --features="std"
cargo test --no-default-features --features="std"

# It should build with both serialization library
for serialize in rustc-serialize serde; do
  cargo build --verbose --no-default-features --features="$serialize"
  cargo test --verbose --no-default-features --features="$serialize"
  cargo build --verbose --no-default-features --features="std,$serialize"
  cargo test --verbose --no-default-features --features="std,$serialize"
done

# Downgrade serde and build test the 0.7.0 channel as well
cargo update -p serde --precise 0.7.0
cargo build --verbose --no-default-features --features "serde"
cargo build --verbose --no-default-features --features "std,serde"
