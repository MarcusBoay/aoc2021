#!/bin/bash

# Usage: ./x.sh <puzzle number>
# Example: ./x.sh 14 # makes template for 14.rs, tests/14/1.txt, ...

PN=$1

# Make .rs
cp src/template.rs src/"${PN}".rs

# Add bin to Cargo.toml
cat << EOF >> Cargo.toml

[[bin]]
name = "${PN}"
path = "src/${PN}.rs"
EOF

# Make test files.
mkdir -p tests/"${PN}"
touch tests/"${PN}"/1.txt
touch tests/"${PN}"/2.txt