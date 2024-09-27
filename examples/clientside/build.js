import fs from "fs"
import { build } from "../shared/build.js"

await build({
  esbuildOptions: {
    entryPoints: ["../shared/run_hello_world.js"],
    outdir: "../altv-server/resources/rust/client",
    external: [
      "alt-shared",
      "alt-client",

      // wasm js bindings code has conditional import of it
      "fs/promises"
    ],
  },
  pluginOptions: {
    target: "client",
    wasmPathForClientRead: "/client/rust_wasm_bg.wasm"
  }
})

const SERVER_JS = "../altv-server/resources/rust/server.js"
if (!fs.existsSync(SERVER_JS)) {
  fs.writeFileSync(SERVER_JS, "")
}
