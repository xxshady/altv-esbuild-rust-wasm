import type * as esbuild from "esbuild"
import fs from "fs"
import { PLUGIN_NAME } from "./const"
import path from "path"

export const altvEsbuildRustWasm = (): esbuild.Plugin => {
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

        const jsFilePath = path.resolve(
          path.dirname(wasmPath),
          path.basename(wasmPath, ".wasm")
            .slice(0, -3) + // removing _bg at the end
            ".js",
        )

        const wasmBinary = fs.readFileSync(wasmPath)
        const wasmBinaryString = wasmBinary.reduce((acc, value) => {
          return acc + value.toString() + "|"
        }, "")

        let jsFileContent = fs.readFileSync(jsFilePath).toString()

        jsFileContent = jsFileContent
          .replace("(function() {", "")
          .replace("})();", "")

        return {
          contents: `
            function decodeWasmBinaryString(bytes) {
              bytes = bytes.split("|").slice(0, -1).map(Number)
              return new Uint8Array(bytes).buffer
            }

            const wasmLoader = (altv_imports) => {
              // ------- jsFileContent
              ${jsFileContent}
              // ------- jsFileContent

              const wasmArrayBuffer = decodeWasmBinaryString("${wasmBinaryString}");
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
