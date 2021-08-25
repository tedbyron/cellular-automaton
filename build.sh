#!/bin/bash

red="\033[0;31m"
green="\033[0;32m"
nc="\033[0m"
commands=(
    "sed"
    "wasm-pack"
    "wasm-opt"
)

command_exists() {
    if [[ -x "$(command -v "$1")"  ]]; then
        echo -e "$1 ${green}installed${nc}"
    else
        echo -e "$1 ${red}not installed${nc}" && exit 1
    fi
}

for cmd in "${commands[@]}"
do
    command_exists $cmd
done

wasm-pack build --scope tedbyron --out-name ca
rm "./pkg/.gitignore"
{
    sed -i '' '2s/cellular-automaton/ca/g' "./pkg/package.json" &&
    echo -e "sed -i '' '2s/cellular-automaton/ca/g' \"./pkg/package.json\" ${green}done${nc}"
} || echo -e "${red}error${nc} updating package.json"
{
    wasm-opt -O4 "./pkg/ca_bg.wasm" -o "./pkg/ca_bg.wasm" &&
    echo -e "wasm-opt -O4 \"./pkg/ca_bg.wasm\" -o \"./pkg/ca_bg.wasm\" ${green}done${nc}"
} || echo -e "${red}error${nc} updating ./pkg/ca_bg.wasm"

unset red
unset green
unset nc
unset commands
unset -f command_exists
