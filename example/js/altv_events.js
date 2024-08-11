import alt from "alt-shared"
import {
  get_base_object_generation_id,
  get_server_base_object_generation_id,
} from "./generation_id.js"

/**
 * @param {import('./resource.js').Resource} resource
 * @param {*} event_name
 * @returns
 */
export function enable_altv_event(resource, event_name) {
  /**
   * @type {import("alt-client").IClientEvent}
   */
  const handlers = {
    serverStarted: () => {
      resource.call_export("on_altv_event", { serverStarted: {} })
    },
    consoleCommand: (name, ...args) => {
      resource.call_export("on_altv_event", { consoleCommand: { name, args } })
    },
    baseObjectCreate: (base_object) => {
      if (base_object.getStreamSyncedMeta) {
        alt.log(
          "[baseObjectCreate] ignoring base object with stream synced meta:",
          base_object.constructor.name,
        )
        return
      }

      resource.call_export("on_altv_event", {
        baseObjectCreate: {
          base_object: {
            id: base_object.id,
            btype: base_object.type,
            generation: get_base_object_generation_id(),
          },
        },
      })
    },
    baseObjectRemove: (base_object) => {
      resource.call_export("on_altv_event", {
        baseObjectRemove: {
          base_object: {
            id: base_object.id,
            btype: base_object.type,
            generation: get_base_object_generation_id(),
          },
        },
      })
    },
    gameEntityCreate: (entity) => {
      resource.call_export("on_altv_event", {
        gameEntityCreate: {
          entity,
          generation: entity.isRemote ? get_server_base_object_generation_id(entity) : null,
        },
      })
    },
    gameEntityDestroy: (entity) => {
      resource.call_export("on_altv_event", { gameEntityDestroy: { entity } })
    },
    worldObjectStreamIn: (world_object) => {
      resource.call_export("on_altv_event", {
        worldObjectStreamIn: {
          world_object,
          // virtual entities
          generation: entity.isRemote ? get_server_base_object_generation_id(entity) : null,
        },
      })
    },
    worldObjectStreamOut: (world_object) => {
      resource.call_export("on_altv_event", { worldObjectStreamOut: { world_object } })
    },
  }
  const handler = handlers[event_name]
  if (!handler) {
    alt.logError("unhandled event:", event_name)
    return
  }
  resource.add_event_handler(event_name, handler)
}
