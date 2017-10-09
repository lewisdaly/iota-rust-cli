#!/usr/bin/env bash

cargo build || exit 1

echo $COMMAND
./target/debug/iotawallet \
  -h service.iotasupport.com -p 14265\
  "$@"
