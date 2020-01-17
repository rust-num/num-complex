#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

case "$TRAVIS_RUST_VERSION" in
  1.31.*) FEATURES="serde" ;;
  *) FEATURES="serde rand" ;;
esac

# num-complex should build and test everywhere.
cargo build --verbose
cargo test --verbose

# It should build with minimal features too.
cargo build --no-default-features
cargo test --no-default-features

# Each isolated feature should also work everywhere.
for feature in $FEATURES; do
  cargo build --verbose --no-default-features --features="$feature"
  cargo test --verbose --no-default-features --features="$feature"
done

# test all supported features together
cargo build --features="$FEATURES"
cargo test --features="$FEATURES"

# test all supported features together with std
cargo build --features="std $FEATURES"
cargo test --features="std $FEATURES"
