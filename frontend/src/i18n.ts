import { createI18n } from "vue-i18n";

import de from "./locales/de.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import ro from "./locales/ro.json";

export const SUPPORTED_LOCALES = ["en", "es", "fr", "de", "ro"] as const;
export type SupportedLocale = (typeof SUPPORTED_LOCALES)[number];

/**
 * Detects the user's preferred language from browser settings
 * @returns The detected locale or 'en' as fallback
 */
export function detectBrowserLanguage(): SupportedLocale {
  const browserLanguages = navigator.languages || [navigator.language];

  for (const lang of browserLanguages) {
    if (!lang) {continue;}

    // Extract language code (e.g., 'en' from 'en-US')
    const parts = lang.split("-");
    if (parts.length === 0 || !parts[0]) {continue;}

    const langCode = parts[0].toLowerCase();

    if (SUPPORTED_LOCALES.includes(langCode as SupportedLocale)) {
      return langCode as SupportedLocale;
    }
  }

  // Default to English if no supported language is found
  return "en";
}

const i18n = createI18n({
  legacy: false,
  locale: detectBrowserLanguage(),
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
