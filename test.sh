#!/bin/bash -eu

mkdir -p test_results

echo "Compiling"
RUSTFLAGS=-Awarnings cargo --quiet build

for i in {1..28}
do
  echo "Running $i"
  RUSTFLAGS=-Awarnings cargo --quiet run $i > test_results/$i.txt
done

diff test_results expected || true
rm -rf test_results
