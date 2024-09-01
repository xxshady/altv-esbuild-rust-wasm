import alt from "alt-client"
import { SourceMapConsumer } from "source-map"
;(async () => {
  // TODO: add copy to build.js
  const sourcemap = alt.File.read("/wasm.map", "utf-8")

  // TODO: add copy to build.js
  const wasmMappings = alt.File.read("/mappings.wasm", "binary")
  SourceMapConsumer.initialize({ "lib/mappings.wasm": wasmMappings })

  // TODO: get rid of async?
  const consumer = await new SourceMapConsumer(sourcemap)

  Error.prepareStackTrace = (err, frames) => {
    return err.stack.split("\n").map((frameStr, idx) => {
      if (!frameStr.includes("wasm://")) return frameStr

      const frame = frames[idx - 1]
      alt.Utils.assert(frame != null)

      const column = frame.getColumnNumber()
      alt.Utils.assert(column != null)

      const original = consumer.originalPositionFor({ line: 1, column: column - 1 })
      if (original?.line == null) {
        return frameStr
      }

      return frameStr.replace(
        /\(wasm:\/\/.*\)/,
        `(${original.source}:${original.line}:${original.column + 1})`,
      )
    }).join("\n")
  }
})().catch(alt.logError)
