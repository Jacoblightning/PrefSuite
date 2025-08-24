#!/usr/bin/env bash

# Error script on first error
set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <Directory to put app>"
  exit
fi

cargo build --release

mkdir -p "$1/PrefSuite.app/Contents/MacOS/"

cp target/release/PrefSuite "$1/PrefSuite.app/Contents/MacOS"

echo "App: $1/PrefSuite.app"
