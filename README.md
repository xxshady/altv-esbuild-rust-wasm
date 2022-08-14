# altv-esbuild-rust-wasm

esbuild plugin that helps you to use rust with wasm in alt:V on serverside and clientside.

Requirement for building rust to wasm is installed [wasm-pack](https://rustwasm.github.io/docs/wasm-pack).

## How to use?

Example resource: [link](/example/README.md).

### Install package via npm

```cmd
npm i altv-esbuild-rust-wasm
```

### Add to plugins

```js
import esbuild from "esbuild"

esbuild.build({
  // ...
  plugins: [
    altvEsbuildRustWasm(),
  ],
})
```

### Usage in JS code

```js
import loadWasm from "./pkg/example.wasm"

const {
  // these values are exported from rust
  // (see example resource)
  ...wasmExports
} = loadWasm({
  // these values are imported to rust using:
  // #[wasm_bindgen(js_namespace = altv_imports)]
  // (see example resource)
  ...wasmImports, 
})
```

Rust wasm-bingen documention: [exports](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/index.html) and [imports](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/index.html).
