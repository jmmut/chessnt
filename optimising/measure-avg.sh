#!/bin/bash

tee /dev/stderr | tr -d "[a-z] ." | awk 'BEGIN {total=0} {total+=$1} END {printf("%.2f\n", total/NR)}'

