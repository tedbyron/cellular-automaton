<div align="center">
  <h1><code>cellular-automaton</code></h1>

  <p>
    <strong>A cellular automaton simulation library targeting WebAssembly.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/cellular-automaton"><img alt="Crates.io version" src="https://img.shields.io/crates/v/cellular-automaton?style=flat-square&logo=rust"></a>
    <a href="https://crates.io/crates/cellular-automaton"><img alt="Crates.io downloads" src="https://img.shields.io/crates/d/cellular-automaton?style=flat-square&label=crates.io downloads"></a>
    <a href="https://docs.rs/cellular-automaton"><img src="https://img.shields.io/badge/docs.rs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <p>
    <a href="https://www.npmjs.com/package/@tedbyron/cellular-automaton"><img alt="npm version (scoped)" src="https://img.shields.io/npm/v/@tedbyron/ca?style=flat-square&logo=npm"></a>
    <a href="https://www.npmjs.com/package/@tedbyron/cellular-automaton"><img alt="npm type definitions" src="https://img.shields.io/npm/types/@tedbyron/ca?style=flat-square"></a>
  </p>

  <h3>
    <a href="https://docs.rs/cellular-automaton">Rust API Docs</a>
  </h3>
</div>

## [Rust Library](https://crates.io/crates/cellular-automaton)

```toml
# Cargo.toml
[dependencies]
cellular-automaton = "0.1"
```

This crate defaults to supporting WebAssembly. To build the crate without WebAssembly support, don't use the crate's default features:

```sh
cargo build --no-default-features
```

## [JavaScript Package](https://www.npmjs.com/package/@tedbyron/cellular-automaton)

```sh
# npm
npm i @tedbyron/cellular-automaton

# yarn
yarn add @tedbyron/cellular-automaton

# pnpm
pnpm add @tedbyron/cellular-automaton
```

The npm package has:

-   A 60kb WASM binary
-   No dependencies
-   JavaScript bindings to the WASM binary
-   TypeScript definitions

The package is built to target [`webpack`](https://webpack.js.org/) (`--target bundler`) but can be rebuilt from the Rust library for a different target:

```sh
wasm-pack build --target <bundler|nodejs|web|no-modules>
wasm-opt -Os pkg/cellular_automaton_bg.wasm -o pkg/cellular_automaton_bg.wasm
```
