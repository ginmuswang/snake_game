import {
  defineConfig
} from "vite";

export default defineConfig({
  optimizeDeps: {
    exclude: ["snake_game"],
    esbuildOptions: {
      target: "es2020",
    },
  },
});