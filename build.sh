#!/usr/bin/env bash

set -euo pipefail

if [ $# -ne 2 ]; then
	echo "Usage: $0 <output_directory> <target_triple>"
	echo "Example: $0 ~/.config/opendeck/plugins/justmangoou.oaytmd.sdPlugin x86_64-unknown-linux-gnu"
	exit 1
fi

cd pi
deno task build
cd ..

rm -rf "$1"
cp -r assets/ "$1"

cargo build --release
cp target/release/oaytmd "$1/oaytmd-$2"
