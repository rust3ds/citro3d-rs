#!/usr/bin/env bash

set -euxo pipefail

cargo run --package bindgen-citro3d > src/bindings.rs
