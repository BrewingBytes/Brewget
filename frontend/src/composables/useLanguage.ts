import { watch } from "vue";
import { useI18n } from "vue-i18n";

import { useAuthStore } from "@/stores/auth";
import { useSettingsStore } from "@/stores/settings";

/**
 * Composable to manage language based on user settings or browser language
 * 
 * For logged-in users, uses the language from their settings
 * For non-logged users, uses the browser language or falls back to English
 */
export function useLanguage() {
  const { locale } = useI18n();
  const authStore = useAuthStore();
  const settingsStore = useSettingsStore();

  // Watch for changes in authentication status and settings
  watch(
    () => [authStore.isAuthenticated, settingsStore.settings?.language] as const,
    ([isAuthenticated, settingsLanguage]) => {
      if (isAuthenticated && settingsLanguage && typeof settingsLanguage === "string") {
        // User is logged in, use their preferred language
        locale.value = settingsLanguage;
      } else {
        // User is not logged in, use browser language or default
        const browserLang = navigator.language.split("-")[0] || "en";
        const supportedLanguages = ["en", "es", "fr", "de"];
        locale.value = supportedLanguages.includes(browserLang) ? browserLang : "en";
      }
    },
    { immediate: true },
  );

  return { locale };
}
