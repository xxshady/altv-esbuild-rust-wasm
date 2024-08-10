import load_wasm from "../rust-wasm/pkg/rust_wasm_bg.wasm"
import { enable_altv_event } from "./altv_events.js"
import { Resource } from "./resource.js"
import * as script_events from "./script_events.js"

// can be either alt-client or alt-server
import alt from "alt-server"

let resource_instance

const exports = load_wasm({
  log_info(string) {
    alt.log("[rust]", string)
  },
  log_warn(string) {
    alt.logWarning("[rust]", string)
  },
  log_error(string) {
    alt.logError("[rust]", string)
  },
  enable_altv_event(event_name) {
    enable_altv_event(resource_instance, event_name)
  },
  disable_altv_event(event_name) {
    resource_instance.remove_event_handler(event_name)
  },
  get_base_object_ref(btype, id) {
    return alt.BaseObject.getByID(btype, id)
  },
  emit_local_event_rust(event_name, buffer) {
    console.log('emit_local_event_rust', { event_name, buffer });
    alt.emit(event_name, buffer)
  },
  emit_local_event_js(event_name, args) {
    alt.emit(event_name, ...args)
  },

  BaseObject: alt.BaseObject,
  WorldObject: alt.WorldObject,
  Entity: alt.Entity,
  Vehicle: alt.Vehicle,
})
resource_instance = new Resource(exports)
script_events.init(resource_instance)

resource_instance.call_export("main")
resource_instance.call_export("test_altv_events")

resource_instance.add_timer(alt.everyTick(() => {
  resource_instance.call_export("on_every_tick")
}))

// ----------------------------- testing

// resource_instance.call_export("test_base_object")
resource_instance.call_export("test_script_events")

// alt.emit("test")
// alt.emit("test", 1, 2, 3)
// alt.emit("test", 256)

