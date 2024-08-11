import alt from "alt-server"
import { GENERATION_ID_KEY } from "./shared.js"

let current_generation_id = 1n

alt.on("baseObjectCreate", (base_object) => {
  if (base_object.setStreamSyncedMeta) {
    base_object.setStreamSyncedMeta(GENERATION_ID_KEY, current_generation_id)
  }
  else if (base_object.setSyncedMeta) {
    base_object.setSyncedMeta(GENERATION_ID_KEY, current_generation_id)
  }
})

alt.on("baseObjectRemove", () => {
  current_generation_id += 1n
})
