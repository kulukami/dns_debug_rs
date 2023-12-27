#!/bin/bash

sudo rm -rf output/*

mkdir output 
cargo clean
cargo build

cp target/debug/resolve output/.