#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

MINOR_VERSION=$(rustc --version | cut -f 1 | cut -d. -f 2)

FEATURES="std serde"
if [[ $MINOR_VERSION -ge 22 ]]; then  # MSRV of rand feature is 1.22.0
  FEATURES="$FEATURES rand"
fi
if [[ $MINOR_VERSION -ge 26 ]]; then  # MSRV of i128 features is 1.26.0
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
