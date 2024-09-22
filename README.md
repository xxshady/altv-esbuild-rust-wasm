# altv-esbuild-rust-wasm

esbuild plugin that helps you to use rust with wasm in alt:V on serverside and clientside.

Requirement for building rust to wasm is installed [wasm-pack](https://rustwasm.github.io/docs/wasm-pack).

## How to use?

Ready-to-use [examples](./examples).

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
    altvEsbuildRustWasm({
      // or "server", depending on where you need to load WASM binary
      target: "client"

      // absolute path of .wasm file for `alt.File.read`
      // only needed for clientside
      wasmPathForClientRead: "/client/rust_wasm_bg.wasm"
    }),
  ],
})
```

### Usage in JS code

```js
import loadWasm from "./pkg/example.wasm"

const {
  // these values are exported from rust
  // (see examples)
  ...wasmExports
} = loadWasm({
  // these values are imported to rust using:
  // #[wasm_bindgen(js_namespace = altv_imports)]
  ...wasmImports, 
})
```

See `wasm-bingen` documentation: [exports](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/index.html) and [imports](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/index.html).
