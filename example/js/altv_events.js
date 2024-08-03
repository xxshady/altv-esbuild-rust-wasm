import alt from "alt-shared"

/**
 * @param {import('./resource.js').Resource} resource
 * @param {*} event_name
 * @returns
 */
export function enable_altv_event(resource, event_name) {
  if (alt.isServer) {
    /**
     * @type {import("alt-server").IServerEvent}
     */
    const handlers = {
      serverStarted: () => {
        resource.call_export("on_altv_event", { serverStarted: {} })
      },
      consoleCommand: (name, ...args) => {
        resource.call_export("on_altv_event", { consoleCommand: { name, args } })
      },
    }
    const handler = handlers[event_name]
    if (!handler) {
      alt.logError("unhandled event:", event_name)
      return
    }
    resource.add_event_handler(event_name, handler)
  }
  else {
    throw new Error("TODO:")
  }
}
