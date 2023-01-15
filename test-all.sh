#!/usr/bin/env bash

cargo clippy && \
cargo clippy --features serde && \
cargo clippy --features v1 && \
cargo clippy --features v1,serde && \
cargo clippy --features v3 && \
cargo clippy --features v3,serde && \
cargo clippy --features v4 && \
cargo clippy --features v4,serde && \
cargo clippy --features v5 && \
cargo clippy --features v5,serde && \
cargo clippy --features v6 && \
cargo clippy --features v6,serde && \
cargo clippy --features v7 && \
cargo clippy --features v7,serde && \
cargo clippy --features v8 && \
cargo clippy --features v8,serde && \
cargo test && \
cargo test --features serde && \
cargo test --features v1 && \
cargo test --features v1,serde && \
cargo test --features v3 && \
cargo test --features v3,serde && \
cargo test --features v4 && \
cargo test --features v4,serde && \
cargo test --features v5 && \
cargo test --features v5,serde && \
cargo test --features v6 && \
cargo test --features v6,serde && \
cargo test --features v7 && \
cargo test --features v7,serde && \
cargo test --features v8 && \
cargo test --features v8,serde
