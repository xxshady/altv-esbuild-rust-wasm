import altShared from "alt-shared"
import loadWasm from "../rust-wasm/pkg/rust_wasm_bg.wasm"
import { enable_altv_event } from "./altv_events.js"
import { Resource } from "./resource.js"

// can be either alt-client or alt-server
import alt from "alt-server"

let resource_instance

const exports = loadWasm({
  log_info(string) {
    altShared.log("[rust]", string)
  },
  log_warn(string) {
    altShared.logWarning("[rust]", string)
  },
  enable_altv_event(event_name) {
    enable_altv_event(resource_instance, event_name)
  },
  disable_altv_event(event_name) {
    resource_instance.remove_event_handler(event_name)
  },
  BaseObject: alt.BaseObject,
  WorldObject: alt.WorldObject,
  Entity: alt.Entity,
  Vehicle: alt.Vehicle,
})
resource_instance = new Resource(exports)

resource_instance.call_export('main')
resource_instance.call_export('test_altv_events')

resource_instance.add_timer(altShared.everyTick(() => {
  resource_instance.call_export('on_every_tick')
}))

resource_instance.call_export('test_base_object')
