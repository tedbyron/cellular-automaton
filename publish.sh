#!/bin/bash

cargo publish
wasm-pack publish --access public
