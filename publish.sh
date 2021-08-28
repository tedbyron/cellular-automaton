#!/bin/bash

cargo publish --dry-run
wasm-pack publish --access public --dry-run
