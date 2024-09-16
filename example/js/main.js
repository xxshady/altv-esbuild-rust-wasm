import alt from "alt-client"
import load_wasm from "../rust-wasm/pkg/rust_wasm_bg.wasm"
import { enable_altv_event } from "./altv_events.js"
import { Resource } from "./resource.js"
import * as script_events from "./script_events.js"
import "./generation_id.js"
import { get_server_base_object_generation_id } from "./generation_id.js"
import * as sourcemap from "./sourcemap.js"

async function main() {
  await sourcemap.init()

  Error.stackTraceLimit = 100

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
    get_base_object_ref(sdk_type, is_remote, id) {
      if (is_remote) {
        return alt.BaseObject.getByRemoteID(sdk_type, id)
      }
      else {
        return alt.BaseObject.getByID(sdk_type, id)
      }
    },
    emit_local_event_rust(event_name, buffer) {
      console.log("emit_local_event_rust", { event_name, buffer })
      alt.emit(event_name, buffer)
    },
    emit_local_event_js(event_name, args) {
      alt.emit(event_name, ...args)
    },

    // returns Vec<PlayerHandle>
    get_streamed_in_players() {
      // TODO: use streamedIn

      return alt.Player.all.map(p => ({
        id: p.id,
        generation: get_server_base_object_generation_id(p),
      }))
    },

    // TEST
    get_player_name(player) {
      return player.name
    },

    // TEST
    get_entity_model(entity) {
      return entity.model
    },

    get_net_time() {
      return alt.getNetTime()
    },

    BaseObject: alt.BaseObject,
    WorldObject: alt.WorldObject,
    Entity: alt.Entity,
    Vehicle: alt.Vehicle,
  })
  resource_instance = new Resource(exports)
  script_events.init(resource_instance)

  resource_instance.call_export("main")
  // resource_instance.call_export("test_altv_events")

  resource_instance.add_timer(alt.everyTick(() => {
    resource_instance.call_export("on_every_tick")
  }))

  // ----------------------------- testing

  // resource_instance.call_export("test_base_object")
  resource_instance.call_export("test_script_events")
  // resource_instance.call_export("test_timers")
  // resource_instance.call_export("test_timers2")
  // resource_instance.call_export("test_altv_events2")

  // alt.emit("test")
  // alt.emit("test", 1, 2, 3)
  // alt.emit("test", 256)
}

main().catch(alt.logError)
