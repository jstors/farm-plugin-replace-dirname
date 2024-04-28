import { defineConfig } from "@farmfe/core";

export default defineConfig({
  compilation: {
    input: {
      index: "./b.js",
      // index: "./index.html",
    },
    output: {
      targetEnv: "node",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    // ["@farmfe/plugin-react", { runtime: "automatic" }],
    "farm-plugin-replace-dirname",
  ],
});
