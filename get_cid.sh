#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

EFFECTOR_NAME=vm_effector

ipfs add -Q --only-hash --cid-version 1 --hash sha2-256 --chunker=size-262144 "target/wasm32-wasi/release/$EFFECTOR_NAME.wasm"
