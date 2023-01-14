#!/usr/bin/env bash

cargo clippy --no-default-features
for version in v1 v3 v4 v5 v6 v7 v8; do
  cargo clippy --features $version
  cargo test --features $version
  cargo clippy --features $version,serde
  cargo test --features $version,serde
done
