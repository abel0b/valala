#!/usr/bin/env bash

set -e

version=`git tag`
workspace=`pwd`
mkdir -p build

function release {
    workdir=`mktemp -d`
    cargo build --release --target "$1"
    cp -r "$workspace/client/res" "$workspace/client/settings.ron" "$workdir"
    cp "$workspace/target/$1/release/valala-client" "$workdir/$2"
    cd "$workdir"
    tar czf "$workspace/build/valala-$version-$3.tar.gz" res settings.ron $2
}

release "x86_64-unknown-linux-gnu" "valala" "linux"
release "x86_64-pc-windows-gnu" "Valala.exe" "windows"

cp "$workspace/target/x86_64-unknown-linux-gnu/release/valala-server" "$workspace/build/valala-server-$version"
