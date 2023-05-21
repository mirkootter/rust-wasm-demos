# Simple examples for native wasm exceptions with rust
## Important notes
Those examples are designed for the fork https://github.com/mirkootter/rust-wasm-exceptions

To try those examples, build a wasm32-wasi toolchain using this fork and run
```
cargo build --release
```
in the example's directory. The .cargo/config.toml already contains the necessary command line arguments and the target.

## How to run the resulting wasm files?
### NodeJS
You can use the `runner.mjs` script in this repository:
```bash
node runner.mjs hello_panic.wasm 
```
This should result in the following output:
```
`r#try` called with ptr 0x1234
Dropped
Caught something!
  data     : 0x1234
  exception: "Oops"
This program terminates correctly.
```

### In the browser
You can run the resulting wasm binaries in https://webassembly.sh