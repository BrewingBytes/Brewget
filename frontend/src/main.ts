import "@/assets/main.css";

import Aura from "@primeuix/themes/aura";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import { ToastService } from "primevue";
import PrimeVue from "primevue/config";
import { createApp } from "vue";

import App from "@/App.vue";
import i18n from "@/i18n";
import router from "@/router";

const app = createApp(App);
const pinia = createPinia();

pinia.use(piniaPluginPersistedstate);

app.use(pinia);
app.use(router);
app.use(i18n);
app.use(ToastService);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: false || 'none',
    }
  },
});

app.mount("#app");
