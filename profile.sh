#!/bin/bash

cargo t --no-run --profile profiling

samply record -o optimising/profile.json.gz cargo test --lib chessnt --profile profiling -- --no-capture --ignored benchmark
