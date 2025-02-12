#!/bin/bash

export RUSTFLAGS="-C target-cpu=native"

for dir in test/test-*; do
  IFS='_' read -r -a files <<<"$(basename "$dir" | cut -c6-)"
  file_a=${files[0]}
  file_b=${files[1]}

  for result in "$dir"/*; do
    IFS=' ' read -r -a args <<<"$(basename "${result}" | cut -c8-)"

    snapshot_dir="tmp-tests/snapshots/$dir"
    mkdir -p "$snapshot_dir"
    cargo run --release -- "test/__fixtures__/${file_a}.kdbx" "test/__fixtures__/${file_b}.kdbx" "${args[@]}" >"$snapshot_dir/result_${args[*]}"
  done
done
