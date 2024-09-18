import { build } from "esbuild"
import { altvEsbuildRustWasm } from "altv-esbuild-rust-wasm"
import { altvEsbuild } from "altv-esbuild"
import fs from "fs"

await build({
  bundle: true,
  logLevel: "info",
  format: "esm",
  entryPoints: ["./js/main.js"],
  outfile: "./server/resources/rust/client/client.js",
  plugins: [
    altvEsbuild({ mode: 'client', altvEnums: true }),
    altvEsbuildRustWasm(),
  ],
})

fs.copyFileSync("./rust-wasm/pkg/rust_wasm_bg.wasm.map", "./server/resources/rust/client/wasm.map")
fs.copyFileSync("./rust-server/target/debug/rust_server.dll", "./server/resources/rust-server/server.dll")
fs.copyFileSync("./source-map/lib/mappings.wasm", "./server/resources/rust/client/mappings.wasm")

const serverJs = "./server/resources/rust/server.js"
if (!fs.existsSync(serverJs)) {
  fs.writeFileSync(serverJs, "")
}
