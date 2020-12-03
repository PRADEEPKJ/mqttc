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
if [ $result == 0 ]
then
  echo "test passed"
  return 1
else
  echo "test failed"
  return 0
fi
:'
# Build
result=$(cargo build --release)
echo $result
'
