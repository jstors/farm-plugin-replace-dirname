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
  },
  plugins: ["farm-plugin-replace-dirname"],
});
