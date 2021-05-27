# LZ77 compression algorithm

## Table of contents
* [General info](#general-info)
* [How to compile](#how-to-compile)
* [Usage](#usage)
* [Running locally](#running-locally)
* [GitHub Pages](#github-pages)

## General info
Comparing Rust-to-Wasm and JS implementations of the Lempel-Ziv (1977) compression algorithm.

The `src` folder contains the Rust source that can be cross-compiled to a CLI application,
and also the necessary Rust wrapper that enables `wasm-pack` to compile to Wasm.

The `www` folder contains the webapp and the JS implementation. The webapp fetches the Rust logo
and encodes it using both Wasm and JS implementations of LZ77, benchmarking the computation time.

The `pkg` folder contains the output of `wasm-pack`.
This repo comes with a Wasm compiled version of the Rust source, used in the webapp.

## How to compile
Compile Rust to binary with `cargo build --release`.

Compile Rust to WASM with `wasm-pack build --target web`.

## Usage
For the compiled Rust CLI application:
```
wasm-lz77 [file]
```
Running with no parameters will output a dry run on screen.
Running with one parameter will perform the LZ77 compression and output the size (in LZ77 codes).

For the Wasm binary:
```
import init, { encode_8, encode_16, encode_32 } from '../pkg/lz77.js';
```
from a JS module. Once `init` resolves (asynchronously), the imported methods are available and executed on the Wasm VM.

## Running locally
Run the webapp locally with `python serve.py [PORT]` (default 8080), and head to `localhost:PORT`.

## GitHub Pages
The webapp is also deployed at [https://angelorendina.github.io/wasm-lz77/](https://angelorendina.github.io/wasm-lz77/).
