import { defineConfig } from "@farmfe/core";
import replaceDirnamePlugin from 'farm-js-plugin-replace-dirname'
export default defineConfig({
  compilation: {
    input: {
      index: "./index.ts",
    },
    persistentCache: false
  },
  plugins: [
    replaceDirnamePlugin()
  ],
});
