import { URL, fileURLToPath } from "node:url";

import { PrimeVueResolver } from "@primevue/auto-import-resolver";
import tailwind from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { defineConfig } from "vite";
import vueDevTools from "vite-plugin-vue-devtools";

import packageJson from "./package.json";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    tailwind(),
    Components({
      resolvers: [
        PrimeVueResolver(),
      ],
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  define: {
    "import.meta.env.VITE_APP_VERSION": JSON.stringify(packageJson.version),
  },
});
