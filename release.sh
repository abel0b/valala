#!/usr/bin/env bash

rm -rf target/debug/res
cp -r res target/debug
cd target/debug
tar -czf valala.tar.gz res valala
