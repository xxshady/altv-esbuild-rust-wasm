import alt from "alt-shared"
import loadWasm from "./rust-wasm/pkg/rust_wasm_bg.wasm"
import { enable_altv_event } from "./altv_events.js"
import { Resource } from "./resource.js"

let resource_instance

const exports = loadWasm({
  log_info(string) {
    alt.log("[rust]", string)
  },
  log_info(string) {
    alt.logWarning("[rust]", string)
  },
  enable_altv_event(event_name) {
    enable_altv_event(resource_instance, event_name)
  },
  disable_altv_event(event_name) {
    resource_instance.remove_event_handler(event_name)
  }
})
resource_instance = new Resource(exports)

resource_instance.call_export('main')
resource_instance.call_export('test_altv_events')

resource_instance.add_timer(alt.everyTick(() => {
  resource_instance.call_export('on_every_tick')
}))
