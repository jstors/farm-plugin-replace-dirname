import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.js",
    },
    output: {
      format: "cjs"
    },
    progress: false,
    persistentCache: false,
    runtime: {
      isolate: true
    },
    minify: false,
    sourcemap: false,
  },
  plugins: ["farm-plugin-replace-dirname"],
});
