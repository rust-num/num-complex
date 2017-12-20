#!/bin/bash

set -ex

echo Testing num-complex on rustc ${TRAVIS_RUST_VERSION}

# num-complex should build and test everywhere.
cargo build --verbose
cargo test --verbose

# It should build with minimal features too.
cargo build --no-default-features
cargo test --no-default-features

# It also should build with serde features
cargo build --verbose --no-default-features --features=serde
cargo test --verbose --no-default-features --features=serde
