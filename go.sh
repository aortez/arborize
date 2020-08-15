#!/bin/bash
set -euxof pipefail
WINIT_UNIX_BACKEND=x11 cargo run --features "vulkan"
