import esbuild from "esbuild"
import { execSync } from "child_process"
import path from "path"
import { altvEsbuildRustWasm } from "../../dist/main.js"

export const build = async ({ esbuildOptions, pluginOptions }) => {
  execSync("npm run build-wasm", {
    stdio: "inherit",
    cwd: new URL(path.dirname(import.meta.url))
  })

  console.log("building js")

  const ctx = await esbuild.context({
    bundle: true,
    format: "esm",
    target: "esnext",
    logLevel: "info",
    keepNames: true,
    plugins: [
      altvEsbuildRustWasm(pluginOptions)
    ],
    ...esbuildOptions,
  })

  await ctx.rebuild()
  await ctx.dispose()
}
