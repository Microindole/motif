import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { motifVite } from "../../../packages/motif-vite/src/index.ts";

export default defineConfig({
  plugins: [react(), motifVite()],
});
