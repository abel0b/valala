#!/usr/bin/env bash

set -e

version=`git tag`
workspace=`pwd`
mkdir -p build

function make_tar {
    tar czf "$1.tar.gz" -C "$2" .
}

function make_zip {
    cd "$2"
    zip -r "$1.zip" *
}

function release {
    workdir=`mktemp -d`
    cargo build --release --target "$1"
    cp -r "$workspace/client/res" "$workspace/client/settings.ron" "$workdir"
    cp "$workspace/target/$1/release/$2" "$workdir"
    $4 "$workspace/build/valala-$version-$3" "$workdir"
}

release x86_64-unknown-linux-gnu valala-client linux make_tar
release x86_64-pc-windows-gnu valala-client.exe windows make_zip

cp "$workspace/target/x86_64-unknown-linux-gnu/release/valala-server" "$workspace/build/valala-server-$version"
