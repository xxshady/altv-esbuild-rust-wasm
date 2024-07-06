import alt from 'alt-shared'

export class Resource {
  exports = {}

  timers = []
  /**
   * @type {Map<string, Function>}
   */
  event_handlers = new Map()
  /**
   * @type {Set<alt.BaseObject>}
   */
  base_objects = new Set()

  constructor(exports) {
    this.exports = exports
  }

  add_event_handler(event_name, handler) {
    alt.Utils.assert(!this.event_handlers.has(event_name))

    alt.on(event_name, handler)
    this.event_handlers.set(event_name, handler)
  }

  remove_event_handler(event_name) {
    const handler = this.event_handlers.get(event_name)
    alt.Utils.assert(handler != null)

    alt.off(event_name, handler)
    this.event_handlers.delete(event_name)
  }

  add_timer(timer) {
    this.timers.push(timer)
  }

  add_base_object(base_object) {
    this.base_objects.add(base_object)
  }

  call_export(name, ...args) {
    try {
      this.exports[name](...args)
    }
    catch {
      alt.logError(`Export call '${name}' panicked`)
      this.drop()
    }
  }

  drop() {
    for (const timer of this.timers) {
      alt.clearTimer(timer)
    }
    for (const [event_name, handler] of this.event_handlers) {
      alt.off(event_name, handler)
    }
    for (const base_object of this.base_objects) {
      base_object.destroy()
    }
  }
}
