// can be either alt-client or alt-server
import alt from "alt-server"

/**
 * @param {import("./resource.js").Resource} resource
 */
export function init(resource) {
  resource.generic_local_event_handler = (event_name, ...args) => {
    resource.call_export("on_script_local_event", {
      local: true,
      name: event_name,
      args,
    })
  }
  alt.on(resource.generic_local_event_handler)

  // TODO: remote script events
  // resource.generic_remote_event_handler = (event_name, ...args) => {
  //   resource.call_export("on_script_event", {
  //     local: false,
  //     name: event_name,
  //     args,
  //   })
  // }
  // alt.onServer(resource.generic_remote_event_handler)
}
