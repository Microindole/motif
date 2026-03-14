import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { motifVite } from "../../../packages/motif-vite/src/index.ts";

export default defineConfig({
  plugins: [vue(), motifVite()],
});
