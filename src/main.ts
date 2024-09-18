import type * as esbuild from "esbuild"
import fs from "fs"
import { PLUGIN_NAME } from "./const"
import path from "path"
import assert from "assert"

export interface Options {
  /**
   * Absolute path of .wasm file for `alt.File.read` (for example `'/client/rust_wasm_bg.wasm'`)
   */
  wasmPathForClientRead: string
}

export const altvEsbuildRustWasm = ({ wasmPathForClientRead }: Options): esbuild.Plugin => {
  assert.ok(wasmPathForClientRead != null, "Expected wasmPathForClientRead option")

  return {
    name: PLUGIN_NAME,
    setup(build): void {
      const namespace = PLUGIN_NAME

      build.onResolve({ filter: /\.wasm$/ }, ({ path, resolveDir }) => {
        return {
          path,
          namespace,
          pluginData: resolveDir,
        }
      })

      build.onLoad({ filter: /.*/, namespace }, ({ path: userPath, pluginData: resolveDir }) => {
        const wasmPath = path.resolve(resolveDir, userPath)

        const outdir = build.initialOptions.outdir
        assert.ok(outdir != null, "esbuild outdir option must present to copy .wasm to it")
        const targetWasmPath = path.join(outdir, path.basename(wasmPath))
        console.log("copying", wasmPath, "->", targetWasmPath)
        fs.copyFileSync(wasmPath, targetWasmPath)

        const jsFilePath = path.resolve(
          path.dirname(wasmPath),
          path.basename(wasmPath, ".wasm")
            .slice(0, -3) + // removing _bg at the end
            ".js",
        )
        let jsFileContent = fs.readFileSync(jsFilePath).toString()

        jsFileContent = jsFileContent
          .replace("(function() {", "")
          .replace("})();", "")

        return {
          contents: `
            import alt from "alt-client";
            const wasmLoader = (altv_imports) => {
              // ------- jsFileContent
              ${jsFileContent}
              // ------- jsFileContent

              const wasmArrayBuffer = alt.File.read("${wasmPathForClientRead}", "binary");
              initSync(wasmArrayBuffer);

              return wasm_bindgen;
            };
            export default wasmLoader;
          `,
          loader: "js",
        }
      })
    },
  }
}
