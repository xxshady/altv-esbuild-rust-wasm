import type * as esbuild from "esbuild"
import fs from "fs"
import { PLUGIN_NAME } from "./const"
import path from "path"

export const altvEsbuildRustWasm = (): esbuild.Plugin => {
  return {
    name: PLUGIN_NAME,
    setup(build): void {
      const textEncoderPolyfill = fs
        .readFileSync(new URL("../text-encoder/polyfill.js", import.meta.url))
        .toString()

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

        const wasmBase64 = fs.readFileSync(wasmPath).toString("base64")
        let jsFileContent = fs.readFileSync(jsFilePath).toString()

        jsFileContent = jsFileContent
          .replace("(function() {", "")
          .replace("})();", "")

        return {
          contents: `
            function base64ToArrayBuffer(base64) {
              const binaryString = atob(base64);
              const len = binaryString.length;
              const bytes = new Uint8Array(len);
              for (let i = 0; i < len; i++) {
                  bytes[i] = binaryString.charCodeAt(i);
              }
              return bytes.buffer;
            }

            function atob(string) {
              const b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=",
                b64re = /^(?:[A-Za-z\\d+\\/]{4})*?(?:[A-Za-z\\d+\\/]{2}(?:==)?|[A-Za-z\\d+\\/]{3}=?)?$/;

              string = String(string).replace(/[\\t\\n\\f\\r ]+/g, "");
              if (!b64re.test(string))
                  throw new TypeError("Failed to execute 'atob': The string to be decoded is not correctly encoded.");
            
              string += "==".slice(2 - (string.length & 3));
              let bitmap, result = "", r1, r2, i = 0;
              for (; i < string.length;) {
                  bitmap = b64.indexOf(string.charAt(i++)) << 18 | b64.indexOf(string.charAt(i++)) << 12
                          | (r1 = b64.indexOf(string.charAt(i++))) << 6 | (r2 = b64.indexOf(string.charAt(i++)));
            
                  result += r1 === 64 ? String.fromCharCode(bitmap >> 16 & 255)
                          : r2 === 64 ? String.fromCharCode(bitmap >> 16 & 255, bitmap >> 8 & 255)
                          : String.fromCharCode(bitmap >> 16 & 255, bitmap >> 8 & 255, bitmap & 255);
              }
              return result;
            }

            ${textEncoderPolyfill}

            const wasmLoader = (altv_imports) => {
              // ------- jsFileContent
              ${jsFileContent}
              // ------- jsFileContent

              const wasmArrayBuffer = base64ToArrayBuffer("${wasmBase64}");
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
