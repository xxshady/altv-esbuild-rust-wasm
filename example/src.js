import alt from "alt-shared"
import loadWasm from "./rust-wasm/pkg/rust_wasm_bg.wasm"

const { call_rust_wasm, on_every_tick } = loadWasm({
  log(string) {
    alt.log("from rust wasm:", string)
  }
})

call_rust_wasm()

const tick = new alt.Utils.EveryTick(() => {
  try {
    on_every_tick();
  } catch (e) {
    tick.destroy()
    alt.log(e)
  }
})
