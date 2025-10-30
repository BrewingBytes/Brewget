import { createI18n } from "vue-i18n";

import de from "./locales/de.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import ro from "./locales/ro.json";

const i18n = createI18n({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
  messages: {
    en,
    es,
    fr,
    de,
    ro,
  },
});

export default i18n;
