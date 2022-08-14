import { build } from "esbuild"
import { altvEsbuildRustWasm } from "altv-esbuild-rust-wasm"

build({
  bundle: true,
  logLevel: "info",
  format: "esm",
  entryPoints: ["./src.js"],
  outfile: "./dist.js",
  external: [
    "alt-shared",
  ],
  plugins: [
    altvEsbuildRustWasm(),
  ],
})
