#!/usr/bin/env bash

set -e

version=`git tag`
workspace=`pwd`
mkdir -p build

function release {
    cargo build --release --target "$1"
    rm -rf target/$1/release/res
    cp -r client/res target/$1/release
    cp client/settings.ron target/$1/release
    cd target/$1/release
    rm -rf $2
    mv valala-client $2
    tar -czf ../../../build/valala-$version-$3.tar.gz res valala settings.ron
}

release "x86_64-unknown-linux-gnu" "valala" "linux"
release "x86_64-pc-windows-gnu" "Valala.exe" "windows"

cp "$workspace/target/x86_64-unknown-linux-gnu/release/valala-server" "$workspace/build/valala-server-$version"
