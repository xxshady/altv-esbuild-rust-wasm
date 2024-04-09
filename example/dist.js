// src.js
import alt from "alt-shared";

// altv-esbuild-rust-wasm:./rust-wasm/pkg/rust_wasm_bg.wasm
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
  const b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=", b64re = /^(?:[A-Za-z\d+\/]{4})*?(?:[A-Za-z\d+\/]{2}(?:==)?|[A-Za-z\d+\/]{3}=?)?$/;
  string = String(string).replace(/[\t\n\f\r ]+/g, "");
  if (!b64re.test(string))
    throw new TypeError("Failed to execute 'atob': The string to be decoded is not correctly encoded.");
  string += "==".slice(2 - (string.length & 3));
  let bitmap, result = "", r1, r2, i = 0;
  for (; i < string.length;) {
    bitmap = b64.indexOf(string.charAt(i++)) << 18 | b64.indexOf(string.charAt(i++)) << 12 | (r1 = b64.indexOf(string.charAt(i++))) << 6 | (r2 = b64.indexOf(string.charAt(i++)));
    result += r1 === 64 ? String.fromCharCode(bitmap >> 16 & 255) : r2 === 64 ? String.fromCharCode(bitmap >> 16 & 255, bitmap >> 8 & 255) : String.fromCharCode(bitmap >> 16 & 255, bitmap >> 8 & 255, bitmap & 255);
  }
  return result;
}
var wasmLoader = (altv_imports) => {
  let wasm_bindgen;
  const __exports = {};
  let wasm;
  const cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
  cachedTextDecoder.decode();
  let cachedUint8Memory0 = new Uint8Array();
  function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
      cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
  }
  function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
  }
  __exports.call_rust_wasm = function () {
    wasm.call_rust_wasm();
  };
  async function load(module, imports) {
    if (typeof Response === "function" && module instanceof Response) {
      if (typeof WebAssembly.instantiateStreaming === "function") {
        try {
          return await WebAssembly.instantiateStreaming(module, imports);
        } catch (e) {
          if (module.headers.get("Content-Type") != "application/wasm") {
            console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
          } else {
            throw e;
          }
        }
      }
      const bytes = await module.arrayBuffer();
      return await WebAssembly.instantiate(bytes, imports);
    } else {
      const instance = await WebAssembly.instantiate(module, imports);
      if (instance instanceof WebAssembly.Instance) {
        return { instance, module };
      } else {
        return instance;
      }
    }
  }
  function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_d43482dbf3b8b693 = function (arg0, arg1) {
      altv_imports.log(getStringFromWasm0(arg0, arg1));
    };
    return imports;
  }
  function initMemory(imports, maybe_memory) {
  }
  function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedUint8Memory0 = new Uint8Array();
    return wasm;
  }
  function initSync(bytes) {
    const imports = getImports();
    initMemory(imports);
    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);
    return finalizeInit(instance, module);
  }
  async function init(input) {
    if (typeof input === "undefined") {
      let src;
      if (typeof document === "undefined") {
        src = location.href;
      } else {
        src = document.currentScript.src;
      }
      input = src.replace(/\.js$/, "_bg.wasm");
    }
    const imports = getImports();
    if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
      input = fetch(input);
    }
    initMemory(imports);
    const { instance, module } = await load(await input, imports);
    return finalizeInit(instance, module);
  }
  wasm_bindgen = Object.assign(init, __exports);
  const wasmArrayBuffer = base64ToArrayBuffer("AGFzbQEAAAABCQJgAn9/AGAAAAIiAQN3YmcaX193YmdfbG9nX2Q0MzQ4MmRiZjNiOGI2OTMAAAMCAQEFAwEAEQcbAgZtZW1vcnkCAA5jYWxsX3J1c3Rfd2FzbQABCg0BCwBBgIDAAEEdEAALCyYBAEGAgMAACx1IZWxsbywgYWx0OlYh8J+kqfCfpK/wn6W28J+YsQB7CXByb2R1Y2VycwIIbGFuZ3VhZ2UBBFJ1c3QADHByb2Nlc3NlZC1ieQMFcnVzdGMdMS43Ni4wICgwN2RjYTQ4OWEgMjAyNC0wMi0wNCkGd2FscnVzBjAuMTkuMAx3YXNtLWJpbmRnZW4SMC4yLjgyICg1OTg4M2VhY2EpACwPdGFyZ2V0X2ZlYXR1cmVzAisPbXV0YWJsZS1nbG9iYWxzKwhzaWduLWV4dA==");
  initSync(wasmArrayBuffer);
  return wasm_bindgen;
};
var rust_wasm_bg_default = wasmLoader;

// src.js
var { call_rust_wasm } = rust_wasm_bg_default({
  log(string) {
    alt.log("from rust wasm:", string);
  }
});
call_rust_wasm();
alt.log('from JS Hello, alt:V!🤩🤯🥶😱')
