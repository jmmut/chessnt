#!/bin/bash

set -euo pipefail

if [ $# != 1 ]; then
  echo "This program receives 1 argument: the new name of this crate"
  echo "This program will rename export_html/index.html and Cargo.toml to replace MY_CRATE_NAME with your new name"
  exit 1
fi

NEW_NAME=$1

sed -i "s/MY_CRATE_NAME/${NEW_NAME}/g" export_html/index.html
sed -i "s/MY_CRATE_NAME/${NEW_NAME}/g" Cargo.toml
sed -i "s/MY_CRATE_NAME/${NEW_NAME}/g" readme.md
sed -i "s/MY_CRATE_NAME/${NEW_NAME}/g" .github/workflows/release.yml
sed -i "s/MY_CRATE_NAME/${NEW_NAME}/g" src/main.rs
