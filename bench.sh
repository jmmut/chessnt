#!/bin/bash

cargo test --profile bench --lib chessnt -- --no-capture --ignored benchmark
