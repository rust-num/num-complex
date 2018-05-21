#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

FEATURES="std rand serde"
if [[ "$TRAVIS_RUST_VERSION" =~ ^(nightly|beta|stable)$ ]]; then
  FEATURES="$FEATURES i128"
fi

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
