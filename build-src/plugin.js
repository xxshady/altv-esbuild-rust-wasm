import esbuild from "esbuild"
import * as shared from "./shared.js"
import { typesGenerator } from "./types-generator.js"

const watch = shared.ESBUILD_OPTIONS.watch && {
  onRebuild: typesGenerator()
}

esbuild.build({
  ...shared.ESBUILD_OPTIONS,
  watch,
  entryPoints: ["src/main.ts"],
  outfile: "dist//main.js",
  platform: 'node',
}).then(typesGenerator())
