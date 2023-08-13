#!/usr/bin/env bash

set -e

CRATE=num-complex
MSRV=1.31

get_rust_version() {
  local array=($(rustc --version));
  echo "${array[1]}";
  return 0;
}
RUST_VERSION=$(get_rust_version)

check_version() {
  IFS=. read -ra rust <<< "$RUST_VERSION"
  IFS=. read -ra want <<< "$1"
  [[ "${rust[0]}" -gt "${want[0]}" ||
   ( "${rust[0]}" -eq "${want[0]}" &&
     "${rust[1]}" -ge "${want[1]}" )
  ]]
}

echo "Testing $CRATE on rustc $RUST_VERSION"
if ! check_version $MSRV ; then
  echo "The minimum for $CRATE is rustc $MSRV"
  exit 1
fi

FEATURES=(libm serde)
check_version 1.34 && FEATURES+=(bytemuck)
check_version 1.36 && FEATURES+=(rand)
check_version 1.54 && FEATURES+=(rkyv/size_64 bytecheck)
echo "Testing supported features: ${FEATURES[*]}"

cargo generate-lockfile

# libm 0.2.6 started using {float}::EPSILON
check_version 1.43 || cargo update -p libm --precise 0.2.5

# Some crates moved to Rust 1.56 / 2021
check_version 1.56 || (
  cargo update -p quote --precise 1.0.30
  cargo update -p proc-macro2 --precise 1.0.65
  cargo update -p rkyv --precise 0.7.40
  cargo update -p bytecheck --precise 0.6.9
)

set -x

# test the default
cargo build
cargo test

# test `no_std`
cargo build --no-default-features
cargo test --no-default-features

# test each isolated feature, with and without std
for feature in ${FEATURES[*]}; do
  cargo build --no-default-features --features="std $feature"
  cargo test --no-default-features --features="std $feature"

  cargo build --no-default-features --features="$feature"
  cargo test --no-default-features --features="$feature"
done

# test all supported features, with and without std
cargo build --features="std ${FEATURES[*]}"
cargo test --features="std ${FEATURES[*]}"

cargo build --features="${FEATURES[*]}"
cargo test --features="${FEATURES[*]}"
