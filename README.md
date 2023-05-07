# Simple examples for native wasm exceptions with rust
## Important notes
Those examples are designed for the fork https://github.com/mirkootter/rust-wasm-exceptions

To try those examples, build a wasm32-wasi toolchain using this fork and run
```
cargo build --release
```
in the example's directory. The .cargo/config.toml already contains the necessary command line arguments and the target.

You can run the resulting wasm binaries in https://webassembly.sh