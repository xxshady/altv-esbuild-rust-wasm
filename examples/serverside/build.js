import { build } from "../shared/build.js"

await build({
  esbuildOptions: {
    platform: "node",
    entryPoints: ["../shared/main.js"],
    outfile: "../altv-server/resources/rust/server.js",
    external: [
      "alt-shared",
    ],
  },
  pluginOptions: {
    target: "server",
  }
})
