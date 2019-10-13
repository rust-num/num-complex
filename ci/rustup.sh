#!/bin/bash
# Use rustup to locally run the same suite of tests as .travis.yml.
# This script installs toolchains used in CI process.

set -ex

export TRAVIS_RUST_VERSION
for TRAVIS_RUST_VERSION in 1.15.0 1.22.0 1.26.0 stable beta nightly; do
    rustup install $TRAVIS_RUST_VERSION
    rustup run $TRAVIS_RUST_VERSION $(cd $(dirname $0);pwd)/test_full.sh
done
