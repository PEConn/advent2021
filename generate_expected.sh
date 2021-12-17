#!/bin/bash -eu

for num in "$@"
do
  echo "cargo run $num > expected/${num}.txt"
  cargo run $num > expected/${num}.txt
done
