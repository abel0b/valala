#!/usr/bin/env bash

set -e

version=`git tag`
workspace=`pwd`
mkdir -p build

function release {
    workdir=`mktemp -d`
    cargo build --release --target "$1"
    cp -r "$workspace/client/res" "$workspace/client/settings.ron" "$workdir"
    cp "$workspace/target/$1/release/$2" "$workdir"
    tar czf "$workspace/build/valala-$version-$3.tar.gz" -C "$workdir" .
}

release "x86_64-unknown-linux-gnu" "valala-client" "linux"
release "x86_64-pc-windows-gnu" "valala-client.exe" "windows"

cp "$workspace/target/x86_64-unknown-linux-gnu/release/valala-server" "$workspace/build/valala-server-$version"
