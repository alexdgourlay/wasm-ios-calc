# WASM iOS Calculator

This rust workspace comprises of 2 crates:
- [ios_calculator](./ios_calculator/) - An implementation of the basic iOS calculator
- [wasm_ios_calculator](./wasm_ios_calculator/) - A wrapper for ios_calculator that compiles to WebAssembly and uses [wasm_bindgen](https://github.com/rustwasm/wasm-bindgen) to expose its methods.


## Build
WASM iOS calculator can be built using [wasm-pack](https://github.com/rustwasm/wasm-pack) by executing the following in the current directory:

```bash
wasm-pack build ./wasm_ios_calculator --out-dir ../pkg
```

This will create a `/pkg` directory that contains the .wasm output along with Typescript bindings.

The `/pkg` directory can then be added as a local dependency for a npm project.