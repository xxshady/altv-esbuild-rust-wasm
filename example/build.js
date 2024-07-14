import { build } from "esbuild"
import { altvEsbuildRustWasm } from "altv-esbuild-rust-wasm"
import { altvEsbuild } from "altv-esbuild"

build({
  bundle: true,
  logLevel: "info",
  format: "esm",
  entryPoints: ["./js/main.js"],
  outfile: "./dist.js",
  external: [
    "alt-shared",
  ],
  plugins: [
    altvEsbuild({ mode: 'server', altvEnums: true }),
    altvEsbuildRustWasm(),
  ],
})
