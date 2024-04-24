import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.js",
    },
    output: {
      targetEnv: "node",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: ["farm-plugin-replace-dirname"],
});
