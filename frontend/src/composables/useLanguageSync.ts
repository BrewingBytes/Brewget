import { watch } from "vue";
import { useI18n } from "vue-i18n";

import { useAuthStore } from "@/stores/auth";
import { useSettingsStore } from "@/stores/settings";

/**
 * Composable to synchronize i18n locale with user settings
 * 
 * This composable:
 * - Sets locale from user settings when authenticated
 * - Falls back to browser language or English when not authenticated
 * - Watches for settings changes and updates locale accordingly
 */
export function useLanguageSync() {
  const { locale } = useI18n();
  const authStore = useAuthStore();
  const settingsStore = useSettingsStore();

  // Get browser language or default to English
  function getBrowserLanguage(): string {
    const supportedLanguages = ["en", "es", "fr", "de", "ro"];
    
    // Try to find first supported language from browser's language preferences
    if (navigator.languages) {
      for (const lang of navigator.languages) {
        const langCode = lang.split("-")[0];
        if (langCode && supportedLanguages.includes(langCode)) {
          return langCode;
        }
      }
    }
    
    // Fallback to navigator.language
    const browserLang = navigator.language?.split("-")[0];
    return browserLang && supportedLanguages.includes(browserLang) ? browserLang : "en";
  }

  // Initialize locale based on auth state
  function initializeLocale() {
    if (authStore.isAuthenticated && settingsStore.settings?.language) {
      // Use user's preferred language from settings
      locale.value = settingsStore.settings.language;
    } else {
      // Use browser language or default to English
      locale.value = getBrowserLanguage();
    }
  }

  // Watch for settings changes
  watch(
    () => settingsStore.settings?.language,
    (newLanguage) => {
      if (newLanguage && authStore.isAuthenticated) {
        locale.value = newLanguage;
      }
    },
  );

  // Watch for authentication state changes
  watch(
    () => authStore.isAuthenticated,
    (isAuthenticated) => {
      if (!isAuthenticated) {
        // Reset to browser language when logged out
        locale.value = getBrowserLanguage();
      } else {
        const userLanguage = settingsStore.settings?.language;
        if (userLanguage) {
          // Apply user's language when logged in
          locale.value = userLanguage;
        }
      }
    },
  );

  return {
    initializeLocale,
  };
}
