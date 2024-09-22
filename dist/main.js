// src/main.ts
import fs from "fs";

// src/const.ts
var PLUGIN_NAME = "altv-esbuild-rust-wasm";

// src/main.ts
import path from "path";
import assert from "assert";
var altvEsbuildRustWasm = ({ target, wasmPathForClientRead }) => {
  assert.ok(target != null, 'Expected target option ("client" or "server")');
  if (target === "client")
    assert.ok(wasmPathForClientRead != null, "Expected wasmPathForClientRead option");
  return {
    name: PLUGIN_NAME,
    setup(build) {
      const namespace = PLUGIN_NAME;
      build.onResolve({ filter: /\.wasm$/ }, ({ path: path2, resolveDir }) => {
        return {
          path: path2,
          namespace,
          pluginData: resolveDir
        };
      });
      build.onLoad({ filter: /.*/, namespace }, ({ path: userPath, pluginData: resolveDir }) => {
        const wasmPath = path.resolve(resolveDir, userPath).replaceAll("\\", "/");
        if (target === "client") {
          const outdir = build.initialOptions.outdir;
          assert.ok(outdir != null, 'esbuild outdir option must present to copy .wasm to it (because target is "client")');
          const targetWasmPath = path.join(outdir, path.basename(wasmPath));
          console.log("copying", wasmPath, "->", targetWasmPath);
          fs.copyFileSync(wasmPath, targetWasmPath);
        }
        const jsFilePath = path.resolve(
          path.dirname(wasmPath),
          path.basename(wasmPath, ".wasm").slice(0, -3) + ".js"
        );
        let jsFileContent = fs.readFileSync(jsFilePath).toString();
        jsFileContent = jsFileContent.replace("(function() {", "").replace("})();", "");
        return {
          contents: target === "client" ? `
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
          ` : `
              import fs from "fs";
              const wasmLoader = (altv_imports) => {
                // ------- jsFileContent
                ${jsFileContent}
                // ------- jsFileContent

                const wasmArrayBuffer = fs.readFileSync("${wasmPath}");
                initSync(wasmArrayBuffer);

                return wasm_bindgen;
              };
              export default wasmLoader;
          `,
          loader: "js"
        };
      });
    }
  };
};
export {
  altvEsbuildRustWasm
};
