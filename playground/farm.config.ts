import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.js",
    },
    progress: false,
    persistentCache: false,
  },
  plugins: ["farm-plugin-replace-dirname"],
});
