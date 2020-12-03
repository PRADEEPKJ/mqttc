#!/usr/bin/bash

#check the format of the rust code
:'
result=$(cargo fmt -- --check)
echo "format $result"

# Lint Checking
result=$(cargo clippy)
echo "clippy $result"
'
# Run tests
result=$(cargo test  --verbose)
if ! cargo test --verbose
then;
  #echo "test passed"
  exit 0
fi
# Build
result=$(cargo build --release)
echo $result
