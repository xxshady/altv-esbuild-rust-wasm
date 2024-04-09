import alt from "alt-shared"
import loadWasm from "./rust-wasm/pkg/rust_wasm_bg.wasm"

const { call_rust_wasm } = loadWasm({
  log(string) {
    alt.log("from rust wasm:", string)
  }
})

call_rust_wasm()
alt.log('from JS Hello, alt:V!🤩🤯🥶😱')
