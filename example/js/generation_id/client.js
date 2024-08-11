import alt from "alt-client"

let current_generation_id = 1n

alt.on("baseObjectCreate", (base_object) => {
  if (base_object.isRemote) return
  base_object.generation_id = current_generation_id
})

alt.on("baseObjectRemove", () => {
  current_generation_id += 1n
})
