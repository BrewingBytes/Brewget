import { createI18n } from "vue-i18n";

import de from "./locales/de.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";

// Get browser language or default to 'en'
function getBrowserLanguage(): string {
  const browserLang = navigator.language.split("-")[0] || "en";
  const supportedLanguages = ["en", "es", "fr", "de"];
  return supportedLanguages.includes(browserLang) ? browserLang : "en";
}

export const i18n = createI18n({
  legacy: false,
  locale: getBrowserLanguage(),
  fallbackLocale: "en",
  messages: {
    en,
    es,
    fr,
    de,
  },
});
