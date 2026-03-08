#!/bin/bash

set -euo pipefail

PROJECT_NAME=chessnt

# Don't run this manually, run local_build_and_run.sh instead to run the project locally.
# This script exists just to make sure the CI and the local building does the same.

# for this script to work, you need to do `rustup target add wasm32-unknown-unknown`

cargo build -r --target wasm32-unknown-unknown
# the folder export_html contains the html wrapper so that the wasm can be used
mkdir -p ${PROJECT_NAME}_html
cp -r export_html/* ${PROJECT_NAME}_html/
cp -r target/wasm32-unknown-unknown/release/*.wasm ${PROJECT_NAME}_html/
cp -r assets ${PROJECT_NAME}_html/
zip -FS -r wasm.zip ${PROJECT_NAME}_html/*
