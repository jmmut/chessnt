#!/bin/bash

cargo t --no-run --profile profiling

mv optimising/profile.json.gz optimising/previous-profile.json.gz

samply record -o optimising/profile.json.gz cargo test --lib chessnt --profile profiling -- --no-capture --ignored benchmark
